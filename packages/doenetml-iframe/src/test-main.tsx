/*
 * This file is for running a dev test.
 * It does not show up in the bundled package.
 */

import React from "react";
import ReactDOM from "react-dom/client";
import { DoenetViewer, DoenetEditor } from "@doenet/doenetml-iframe";

const root = ReactDOM.createRoot(document.getElementById("root")!);
root.render(
    <React.Fragment>
        <h4>DoenetML 0.6:</h4>
        <DoenetViewer
            doenetML={`<p>Use this to test DoenetML</p>
                <graph showNavigation="false">

                  <line through="(-8,8) (9,6)" />
                  <line through="(0,4)" slope="1/2" styleNumber="2" />

                  <line equation="y=2x-8" styleNumber="3" />
                  <line equation="x=-6" styleNumber="4" />

                </graph>`}
            doenetmlVersion="0.6.5"
        />
        <h4>DoenetML 0.7:</h4>
        <DoenetViewer
            doenetML={`<p>Use this to test DoenetML</p>
                <graph showNavigation="false">

                  <line through="(-8,8) (9,6)" />
                  <line through="(0,4)" slope="1/2" styleNumber="2" />

                  <line equation="y=2x-8" styleNumber="3" />
                  <line equation="x=-6" styleNumber="4" />

                </graph>`}
            generatedVariantCallback={(variant: any) =>
                console.log("found variant", variant)
            }
            flags={{ readOnly: true }}
        />
        <h4>DoenetML 0.7 editor:</h4>
        <DoenetEditor
            doenetML={`<p>Use this to test DoenetML</p>
                <graph showNavigation="false">

                  <line through="(-8,8) (9,6)" />
                  <line through="(0,4)" slope="1/2" styleNumber="2" />

                  <line equation="y=2x-8" styleNumber="3" />
                  <line equation="x=-6" styleNumber="4" />

                </graph>`}
            doenetmlVersion="0.7.0-alpha10"
            doenetmlChangeCallback={(doenetml: any) =>
                console.log("new doenetml", doenetml)
            }
        />
    </React.Fragment>,
);
