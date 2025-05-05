import React from "react";
import { BasicComponentWithPassthroughChildren } from "../types";
import type { PPropsInText } from "@doenet/doenetml-worker";

export const Em: BasicComponentWithPassthroughChildren<{
    props: PPropsInText;
}> = ({ children }) => {
    return <em>{children}</em>;
};
