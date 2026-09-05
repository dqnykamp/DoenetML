import { describe, expect, it, vi } from "vitest";
import { getGraphRendererState, getWarnings } from "./graph-prefigure.helpers";
import { createTestCore } from "../utils/test-core";
import { getDiagnosticsByType } from "../utils/diagnostics";

const Mock = vi.fn();
vi.stubGlobal("postMessage", Mock);
vi.mock("hyperformula");

/** The PreFigure XML `<barChart name="c">` produces. */
async function chartXML(
    doenetML: string,
    options: { theme?: "dark" | "light" } = {},
) {
    return (await getGraphRendererState(doenetML, "c", options)).prefigureXML;
}

const FOUR_BARS = `
    <barChart name="c" categories="North South East West" type="text">
      <shortDescription>Counts by region</shortDescription>
      <number>41</number><number>63</number><number>18</number><number>78</number>
    </barChart>
    `;

describe("barChart prefigure tests @group4", async () => {
    describe("diagram shape", async () => {
        it("emits one rectangle and one tick mark per bar", async () => {
            const xml = await chartXML(FOUR_BARS);

            expect(xml.match(/<rectangle /g)?.length).eq(4);
            expect(xml.match(/<tick-mark /g)?.length).eq(4);

            // Bars sit at x = 1..4, 0.8 of their slot wide, so the first spans
            // 0.6 to 1.4.
            expect(xml).toContain(
                '<rectangle at="bar-1" lower-left="(0.6,0)" dimensions="(0.8,41)"',
            );
            expect(xml).toContain(
                '<rectangle at="bar-4" lower-left="(3.6,0)" dimensions="(0.8,78)"',
            );
        });

        it("puts the category names on the horizontal axis", async () => {
            const xml = await chartXML(FOUR_BARS);

            // The whole reason this renders through PreFigure: `hlabels` is a
            // numeric (start, step, end) triple and cannot carry names, so the
            // categories go through <tick-mark> instead.
            expect(xml).toContain('<tick-mark axis="horizontal" location="1"');
            expect(xml).toContain(">North</tick-mark>");
            expect(xml).toContain(">West</tick-mark>");
        });

        it("suppresses automatic labels but keeps numeric ones on the vertical axis", async () => {
            const xml = await chartXML(FOUR_BARS);

            // `decorations="no"` switches off the automatic labels on *both*
            // axes; the explicit vlabels brings them back on the vertical one,
            // leaving the horizontal axis to the tick marks.
            expect(xml).toContain('decorations="no"');
            expect(xml).toContain('vlabels="(0,20,80)"');
        });

        it("reserves margins so the axis labels are not clipped", async () => {
            const xml = await chartXML(FOUR_BARS);

            // Both axes sit on the edge of the bounding box, so without
            // margins their labels would fall outside the drawing area.
            expect(xml).toContain('margins="[46,30,12,16]"');

            // The margins are added around `dimensions`, so the inner size is
            // shrunk by them to keep the chart the size that was asked for.
            expect(xml).toContain('dimensions="(367,237.33333333333331)"');
        });

        it("describes every bar in the annotations", async () => {
            const xml = await chartXML(FOUR_BARS);

            expect(xml).toContain(
                '<annotation ref="figure" text="Counts by region">',
            );
            expect(xml).toContain(
                '<annotation ref="bar-2" text="South: 63" />',
            );
        });
    });

    describe("vertical scale", async () => {
        it("rounds the top up to the next tick above the tallest bar", async () => {
            // 78 rounds to 80 rather than touching the top of the box.
            expect(await chartXML(FOUR_BARS)).toContain('bbox="(0,0,4.5,80)"');
        });

        it("never lets the tallest bar reach the top", async () => {
            // 80 is already a multiple of the step, so the box goes one step
            // further rather than clipping the bar against the frame.
            const xml = await chartXML(`
    <barChart name="c"><number>80</number></barChart>
    `);
            expect(xml).toContain('bbox="(0,0,1.5,100)"');
        });

        it("honors an explicit yMax", async () => {
            const xml = await chartXML(`
    <barChart name="c" yMax="100"><number>41</number><number>63</number></barChart>
    `);
            expect(xml).toContain('bbox="(0,0,2.5,100)"');
        });

        it("gives an empty chart a box one tick tall", async () => {
            // Zeros rather than nothing: an empty chart should read as empty,
            // not as broken.
            const xml = await chartXML(`<barChart name="c" />`);
            expect(xml).toContain('bbox="(0,0,0.5,1)"');
            expect(xml).not.toContain("<rectangle ");
        });

        it("drops the floor below zero for a negative value", async () => {
            const xml = await chartXML(`
    <barChart name="c"><number>5</number><number>-3</number></barChart>
    `);
            // One tick of room past the extremes on both sides.
            expect(xml).toContain('bbox="(0,-4,2.5,6)"');
            // A negative bar hangs from the axis rather than growing from it.
            expect(xml).toContain(
                '<rectangle at="bar-2" lower-left="(1.6,-3)" dimensions="(0.8,3)"',
            );
        });

        it("labels the vertical axis on multiples of the step, including zero", async () => {
            // Anchoring the run at yMin instead would label this chart at
            // -4, -2, 0 ... only by luck; with an odd floor it would never
            // mark the axis the bars are measured from.
            const xml = await chartXML(`
    <barChart name="c"><number>5</number><number>-3</number></barChart>
    `);
            expect(xml).toContain('vlabels="(-4,2,6)"');
        });
    });

    describe("labels and values", async () => {
        it("numbers the bars when no categories are named", async () => {
            const xml = await chartXML(`
    <barChart name="c"><number>4</number><number>7</number></barChart>
    `);
            expect(xml).toContain(">1</tick-mark>");
            expect(xml).toContain(">2</tick-mark>");
        });

        it("prints the value above each bar when asked", async () => {
            const xml = await chartXML(`
    <barChart name="c" displayValues><number>41</number></barChart>
    `);
            expect(xml).toContain('<label anchor="(1,41)" alignment="north"');
            expect(xml).toContain(">41</label>");
        });

        it("prints no values by default", async () => {
            expect(await chartXML(FOUR_BARS)).not.toContain("<label ");
        });

        it("carries the axis labels", async () => {
            const xml = await chartXML(`
    <barChart name="c">
      <xLabel>region</xLabel>
      <yLabel>count</yLabel>
      <number>4</number>
    </barChart>
    `);
            expect(xml).toContain("<xlabel");
            expect(xml).toContain(">region</xlabel>");
            expect(xml).toContain(">count</ylabel>");
        });
    });

    describe("bar width", async () => {
        it("narrows the bars", async () => {
            const xml = await chartXML(`
    <barChart name="c" barWidth="0.5"><number>4</number></barChart>
    `);
            expect(xml).toContain(
                '<rectangle at="bar-1" lower-left="(0.75,0)" dimensions="(0.5,4)"',
            );
        });

        it("warns and falls back when the width is not a fraction of a slot", async () => {
            const { warnings } = await getWarnings(`
    <barChart name="c" barWidth="3"><number>4</number></barChart>
    `);
            expect(
                warnings.some((w) =>
                    w.message.includes("`barWidth` must be greater than 0"),
                ),
            ).eq(true);
        });
    });

    describe("theme", async () => {
        it("lightens the axes in dark mode", async () => {
            // PreFigure defaults axes and ticks to black, which disappears on
            // the dark canvas.
            const xml = await chartXML(FOUR_BARS, { theme: "dark" });
            expect(xml).toContain('stroke="#ffffff"');
        });

        it("leaves them alone in light mode", async () => {
            const xml = await chartXML(FOUR_BARS, { theme: "light" });
            expect(xml).not.toContain('stroke="#ffffff"');
        });
    });

    describe("public state variables", async () => {
        it("exposes the values and the categories", async () => {
            const { core, resolvePathToNodeIdx } = await createTestCore({
                doenetML: `
    ${FOUR_BARS}
    <p name="pv">$c.barValues</p>
    <p name="pc">$c.categories</p>
    `,
            });
            const sv = await core.returnAllStateVariables(false, true);
            expect(sv[await resolvePathToNodeIdx("pv")].stateValues.text).eq(
                "41, 63, 18, 78",
            );
            expect(sv[await resolvePathToNodeIdx("pc")].stateValues.text).eq(
                "North, South, East, West",
            );
        });
    });

    describe("accessibility", async () => {
        it("asks for a short description when there is none", async () => {
            const { core } = await createTestCore({
                doenetML: `<barChart name="c"><number>4</number></barChart>`,
            });
            await core.returnAllStateVariables(false, true);
            const { accessibility } = getDiagnosticsByType(core);
            expect(
                accessibility.some((a) => a.message.includes("barChart")),
            ).eq(true);
        });

        it("stays quiet for a decorative chart", async () => {
            const { core } = await createTestCore({
                doenetML: `<barChart name="c" decorative><number>4</number></barChart>`,
            });
            await core.returnAllStateVariables(false, true);
            const { accessibility } = getDiagnosticsByType(core);
            expect(accessibility.length).eq(0);
        });
    });

    describe("driven by the counting operators", async () => {
        it("charts a tally of sampled subpopulations", async () => {
            // The end of the road this whole family was built for: the chart's
            // children are another composite's replacements.
            const xml = await chartXML(`
    <setup>
      <numberList name="pop">30 45 12 60</numberList>
      <textList name="labels">North South East West</textList>
      <cumulativeSum name="cum">$pop</cumulativeSum>
    </setup>
    <numberList name="draws">5 40 80 100 20 90 76 3</numberList>
    <searchSorted name="which" target="$draws" hide>$cum</searchSorted>
    <tally name="counts" categories="1 2 3 4" hide>$which</tally>
    <barChart name="c" categories="$labels" type="text">
      <shortDescription>Sampled counts</shortDescription>
      $counts
    </barChart>
    `);

            // counts are 3, 1, 2, 2
            expect(xml).toContain('dimensions="(0.8,3)"');
            expect(xml).toContain('<annotation ref="bar-1" text="North: 3" />');
            expect(xml).toContain('<annotation ref="bar-4" text="West: 2" />');
        });
    });
});
