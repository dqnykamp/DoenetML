import { DoenetML } from '@doenet/doenetml/doenetml-inline-worker.js';
import { default as React_2 } from 'react';
import { default as ReactDOM_2 } from 'react-dom/client';

export { DoenetML }

export { React_2 as React }

export { ReactDOM_2 as ReactDOM }

/**
 * Render DoenetML to a container element. If `doenetMLSource` is not provided,
 * it is assumed that `container` has a `<script type="text/doenetml">` child which
 * stores the source.
 */
export declare function renderDoenetToContainer(container: Element, doenetMLSource?: string, config?: object): void;

export { }
