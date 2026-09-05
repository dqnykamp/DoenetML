import React, { useRef } from "react";
import useDoenetRenderer, {
    UseDoenetRendererProps,
} from "../useDoenetRenderer";
import GraphFrame from "./GraphFrame";
import Prefigure from "./prefigure";
import type { GraphSVs } from "./graph";

/**
 * `<barChart>` renders through PreFigure, which is already a general
 * "compile this diagram XML" renderer: its whole input is `prefigureXML`.
 *
 * So this is a frame and a hand-off. `GraphFrame` supplies the sizing, border,
 * background and accessible description that `<graph>` gets, which is why
 * `<barChart>` declares the same handful of framing state variables rather
 * than inventing its own.
 */
export default React.memo(function BarChart(props: UseDoenetRendererProps) {
    const { id, SVs } = useDoenetRenderer<GraphSVs>(props);

    const containerRef = useRef<HTMLDivElement | null>(null);

    return (
        <GraphFrame
            id={id}
            SVs={SVs}
            isPrefigureRenderer={true}
            containerRef={containerRef}
            descriptionChild={false}
            hasInteractiveControls={false}
        >
            {(surfaceStyle) => (
                <Prefigure id={id} SVs={SVs} surfaceStyle={surfaceStyle} />
            )}
        </GraphFrame>
    );
});
