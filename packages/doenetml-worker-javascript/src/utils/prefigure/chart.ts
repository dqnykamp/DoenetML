import { escapeXml, formatNumber } from "./common";
import { labelMarkup, THEME_AWARE_LABEL_COLOR_ATTR } from "./label";
import { styleAttributes } from "./style";
import type { DiagnosticRecord } from "@doenet/utils";

/**
 * PreFigure assembly for `<barChart>`.
 *
 * Kept apart from `graph.ts` because a chart is not a graph with bars in it: it
 * owns its own bounding box, its horizontal axis is categorical rather than
 * numeric, and it has no graphical descendants to convert. What it shares with
 * `graph.ts` is the vocabulary — `common.ts` for escaping and formatting,
 * `style.ts` for Doenet styles, `label.ts` for axis labels — not the algorithm.
 *
 * Two PreFigure elements are emitted here that nothing else in this folder
 * emits yet:
 *
 * - `<tick-mark>`, which places arbitrary text at an arbitrary axis position.
 *   This is the only way to get categorical labels: PreFigure's own `hlabels`
 *   is a numeric `(start, step, end)` triple (`axes.py`), so category names
 *   cannot go through it. Automatic labels are switched off with
 *   `decorations="no"` and the vertical axis gets an explicit `vlabels` back,
 *   leaving the horizontal axis to the tick marks below.
 * - `<label>`, for the optional value printed above each bar.
 *
 * Both axes sit on the edge of the bounding box — the vertical one at x = 0,
 * the horizontal one at the baseline — so their labels would be drawn outside
 * the drawing area and clipped. `<diagram margins>` is the fix: PreFigure adds
 * the margins *outside* `dimensions`, so the inner size is shrunk by them to
 * keep the rendered chart the size the author actually asked for.
 */

/**
 * Room reserved outside the plotting area, in pixels, as
 * `[left, bottom, right, top]`: the left for the count labels, the bottom for
 * the category names, the top for the optional value above the tallest bar.
 */
const CHART_MARGINS = [46, 30, 12, 16] as const;

/** The bar geometry a chart renders, in data coordinates. */
export type BarGeometry = {
    /** 1-based position along the categorical axis. */
    center: number;
    label: string;
    value: number;
    lowerLeft: [number, number];
    dimensions: [number, number];
};

export type BarChartGeometry = {
    bars: BarGeometry[];
    /** `[xMin, yMin, xMax, yMax]` in data coordinates. */
    bounds: [number, number, number, number];
    /** The spacing between labeled values on the vertical axis. */
    tickStep: number;
};

// Matches `graph.ts`: PreFigure defaults axes and ticks to black, which
// disappears on the dark canvas.
const PREFIGURE_DARK_AXIS_COLOR = "#ffffff";

/**
 * A tick step that divides `span` into a handful of intervals and lands on
 * numbers a reader recognizes — 1, 2, 5 and their powers of ten, the same
 * ladder every plotting library climbs.
 *
 * Counts are whole numbers, so the step never goes below 1: a bar chart of
 * counts labeled 0, 0.5, 1 would invite reading half a thing.
 */
export function niceTickStep(span: number, targetIntervals = 5): number {
    if (!Number.isFinite(span) || span <= 0) {
        return 1;
    }

    const rough = span / targetIntervals;
    const magnitude = 10 ** Math.floor(Math.log10(rough));
    const normalized = rough / magnitude;

    let step;
    if (normalized <= 1) {
        step = 1;
    } else if (normalized <= 2) {
        step = 2;
    } else if (normalized <= 5) {
        step = 5;
    } else {
        step = 10;
    }

    return Math.max(1, step * magnitude);
}

/**
 * The bar rectangles and bounding box for a list of values.
 *
 * Bars sit at x = 1, 2, … n and are `barWidth` of their one-unit slot wide, so
 * the gap between them is what is left over. The box starts at x = 0 so the
 * vertical axis has somewhere to be drawn, and ends half a unit past the last
 * bar so the outermost bars are not flush against the frame.
 *
 * `yMax` is the author's when they gave one; otherwise it is rounded up to the
 * next tick so the tallest bar does not touch the top of the box. An empty
 * chart, or one whose values are all zero, still gets a box one tick tall —
 * otherwise the axis would collapse and the chart would look broken rather
 * than empty.
 */
export function computeBarChartGeometry({
    values,
    labels,
    barWidth,
    yMinAttr,
    yMaxAttr,
}: {
    values: number[];
    labels: string[];
    barWidth: number;
    yMinAttr: number | null;
    yMaxAttr: number | null;
}): BarChartGeometry {
    const finiteValues = values.map((value) =>
        Number.isFinite(value) ? value : 0,
    );

    const largest = finiteValues.length > 0 ? Math.max(...finiteValues) : 0;
    const smallest = finiteValues.length > 0 ? Math.min(...finiteValues) : 0;

    // The bars are measured from zero, so zero is always in view even when
    // every value is on one side of it.
    const reachAbove = Math.max(0, largest);
    const reachBelow = Math.min(0, smallest);

    const tickStep = niceTickStep(reachAbove - reachBelow || 1);

    // One tick of headroom past the tallest bar, so it never touches the
    // frame; the same below when any value is negative. A chart with nothing
    // in it still gets one tick of height rather than collapsing.
    const yMax =
        yMaxAttr ??
        (reachAbove <= 0 ? tickStep : nextTickBeyond(reachAbove, tickStep, 1));
    const yMin =
        yMinAttr ??
        (reachBelow >= 0 ? 0 : nextTickBeyond(reachBelow, tickStep, -1));

    const halfWidth = barWidth / 2;

    const bars = finiteValues.map((value, ind) => {
        const center = ind + 1;
        return {
            center,
            label: labels[ind] ?? String(center),
            value,
            lowerLeft: [center - halfWidth, Math.min(0, value)] as [
                number,
                number,
            ],
            dimensions: [barWidth, Math.abs(value)] as [number, number],
        };
    });

    return {
        bars,
        bounds: [0, yMin, finiteValues.length + 0.5, yMax],
        tickStep,
    };
}

/**
 * The first multiple of `step` at or beyond `value` in `direction`, moving one
 * further when `value` already sits exactly on one.
 *
 * That last part is what keeps the tallest bar off the frame: a chart of a
 * single value of 80 with a step of 20 goes to 100, not to 80.
 */
function nextTickBeyond(value: number, step: number, direction: 1 | -1) {
    const rounded =
        direction === 1
            ? Math.ceil(value / step) * step
            : Math.floor(value / step) * step;
    return rounded === value ? rounded + direction * step : rounded;
}

/**
 * Builds the PreFigure XML for a bar chart.
 *
 * The vertical axis keeps numeric labels via an explicit `vlabels`; the
 * horizontal axis has its automatic labels suppressed and gets one
 * `<tick-mark>` per category instead.
 */
export function createBarChartPrefigureXML({
    geometry,
    widthPx,
    heightPx,
    xLabel,
    xLabelHasLatex,
    yLabel,
    yLabelHasLatex,
    selectedStyle,
    displayValues,
    shortDescription,
    darkMode = false,
}: {
    geometry: BarChartGeometry;
    widthPx: number;
    heightPx: number;
    xLabel?: string;
    xLabelHasLatex?: boolean;
    yLabel?: string;
    yLabelHasLatex?: boolean;
    selectedStyle: Record<string, unknown> | undefined;
    displayValues: boolean;
    shortDescription?: string;
    darkMode?: boolean;
}): { xml: string; diagnostics: DiagnosticRecord[] } {
    const diagnostics: DiagnosticRecord[] = [];

    const [xMin, yMin, xMax, yMax] = geometry.bounds;
    const bbox = `(${formatNumber(xMin)},${formatNumber(yMin)},${formatNumber(xMax)},${formatNumber(yMax)})`;

    const [marginLeft, marginBottom, marginRight, marginTop] = CHART_MARGINS;
    // The margins are added around `dimensions`, so shrink it by them to keep
    // the chart the size the frame reserved for it. A chart small enough for
    // the margins to swallow it keeps a positive inner size rather than
    // collapsing.
    const innerWidth = Math.max(widthPx - marginLeft - marginRight, 1);
    const innerHeight = Math.max(heightPx - marginBottom - marginTop, 1);
    const dimensions = `(${formatNumber(innerWidth)},${formatNumber(innerHeight)})`;
    const margins = `[${CHART_MARGINS.join(",")}]`;

    const strokeAttr = darkMode ? ` stroke="${PREFIGURE_DARK_AXIS_COLOR}"` : "";

    // `decorations="no"` suppresses the automatic labels on both axes; the
    // explicit `vlabels` brings them back on the vertical one only.
    //
    // The run starts at the first multiple of the step inside the box rather
    // than at `yMin`, so the labeled values are multiples of the step and one
    // of them lands on zero. Anchoring at `yMin` instead would label a chart
    // running from -3 at -3, -1, 1, 3 — never marking the axis the bars are
    // measured from.
    const step = geometry.tickStep;
    const firstTick = Math.ceil(yMin / step) * step;
    const lastTick = Math.floor(yMax / step) * step;
    const vlabels = `(${formatNumber(firstTick)},${formatNumber(step)},${formatNumber(lastTick)})`;

    const axisLabelElements = [];
    const xLabelText = labelMarkup({
        label: xLabel,
        labelHasLatex: xLabelHasLatex,
    });
    if (xLabelText) {
        axisLabelElements.push(
            `<xlabel alignment="nw" ${THEME_AWARE_LABEL_COLOR_ATTR}>${xLabelText}</xlabel>`,
        );
    }
    const yLabelText = labelMarkup({
        label: yLabel,
        labelHasLatex: yLabelHasLatex,
    });
    if (yLabelText) {
        axisLabelElements.push(
            `<ylabel alignment="se" ${THEME_AWARE_LABEL_COLOR_ATTR}>${yLabelText}</ylabel>`,
        );
    }

    const axesInner = axisLabelElements.join("");
    const axesAttrs = `axes="all" decorations="no" vlabels="${escapeXml(vlabels)}"${strokeAttr}`;
    const axesElement = axesInner
        ? `<axes ${axesAttrs}>${axesInner}</axes>`
        : `<axes ${axesAttrs} />`;

    const barAttrs = styleAttributes({
        selectedStyle,
        diagnostics,
        warningPrefix: "<barChart>",
    }).join(" ");

    const elements: string[] = [];
    const annotationElements: string[] = [];

    for (const [ind, bar] of geometry.bars.entries()) {
        const handle = `bar-${ind + 1}`;
        const lowerLeft = `(${formatNumber(bar.lowerLeft[0])},${formatNumber(bar.lowerLeft[1])})`;
        const barDimensions = `(${formatNumber(bar.dimensions[0])},${formatNumber(bar.dimensions[1])})`;

        elements.push(
            `<rectangle at="${escapeXml(handle)}" lower-left="${escapeXml(lowerLeft)}" dimensions="${escapeXml(barDimensions)}"${barAttrs ? ` ${barAttrs}` : ""} />`,
        );

        // The categorical axis: arbitrary text at an arbitrary position, which
        // is the one thing `hlabels` cannot express.
        elements.push(
            `<tick-mark axis="horizontal" location="${formatNumber(bar.center)}"${strokeAttr} ${THEME_AWARE_LABEL_COLOR_ATTR}>${escapeXml(bar.label)}</tick-mark>`,
        );

        if (displayValues) {
            const anchor = `(${formatNumber(bar.center)},${formatNumber(Math.max(bar.value, 0))})`;
            elements.push(
                `<label anchor="${escapeXml(anchor)}" alignment="north" ${THEME_AWARE_LABEL_COLOR_ATTR}>${escapeXml(formatNumber(bar.value) ?? "")}</label>`,
            );
        }

        annotationElements.push(
            `<annotation ref="${escapeXml(handle)}" text="${escapeXml(`${bar.label}: ${formatNumber(bar.value)}`)}" />`,
        );
    }

    // A figure-level annotation is what diagcess navigates into; without one
    // the per-bar annotations have no parent to hang from. Its text is the
    // author's `<shortDescription>` when there is one — nothing is invented
    // here, so there is no generated English to translate.
    const figureAnnotationText = shortDescription
        ? ` text="${escapeXml(shortDescription)}"`
        : "";
    const annotationsElement = `<annotations><annotation ref="figure"${figureAnnotationText}>${annotationElements.join("")}</annotation></annotations>`;

    const xml = `<diagram dimensions="${escapeXml(dimensions)}" margins="${escapeXml(margins)}"><coordinates bbox="${escapeXml(bbox)}">${axesElement}${elements.join("")}</coordinates>${annotationsElement}</diagram>`;

    return { xml, diagnostics };
}
