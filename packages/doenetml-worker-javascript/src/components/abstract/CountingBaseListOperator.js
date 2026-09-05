import CompositeComponent from "./CompositeComponent";
import me from "math-expressions";
import {
    returnBreakStringsIntoTypeSugarInstruction,
    returnListValueStateVariableDefinitions,
} from "../../utils/listValues";
import { returnListTypeAttribute } from "../../utils/listIndexOperators";
import {
    addReplacementRendererType,
    calculateValueListReplacementChanges,
    createValueListReplacements,
    returnPassThroughAttributeDeclarations,
    returnPassThroughAttributes,
} from "../../utils/valueListReplacements";

/**
 * Base class for the operators that answer *how many*: `<tally>` and
 * `<binCounts>`.
 *
 * These are the missing corner of the list-operator families. `MathBaseOperator`
 * reduces a list to one value, `MathBaseListOperator` maps a list of maths to a
 * list of maths, and `ListIndexBaseListOperator` reports positions. None of them
 * fits an operator whose *input* is a list of arbitrary comparable values and
 * whose *output* is a list of counts — one per category or per bin, with no
 * relationship to the length of the input.
 *
 * Like `<sort>`, the input is read through `utils/listValues`, so what counts as
 * "the same value" here is what `<sort>` would call equal — by construction
 * rather than by coincidence. The output is created fresh, so these follow
 * `MathBaseListOperator` in being composites that build `<math>` replacements in
 * the manner of `<sequence>`, which is what makes `$counts[2]`,
 * `<sum>$counts</sum>` and `<numberList>$counts</numberList>` all work.
 *
 * Subclasses supply `countValues`, which receives `{ values, numeric }` and
 * returns `{ counts, labels, diagnostics }`. `labels` is what the counts are
 * counts *of*; a subclass that has nothing useful to say may return `null` and
 * simply not declare a state variable for them. `diagnostics` is optional, and
 * is raised from here rather than from the subclass so that a message about the
 * data — as opposed to one about the attributes — is raised once the values are
 * in hand.
 */
export default class CountingBaseListOperator extends CompositeComponent {
    static componentType = "_countingListOperator";

    static takesIndex = true;

    static stateVariableToEvaluateAfterReplacements =
        "readyToExpandWhenResolved";

    static allowInSchemaAsComponent = ["math"];

    // Since the operator treats each child as a separate argument,
    // composites with no replacement should be ignored.
    static descendantCompositesMustHaveAReplacement = false;

    static createAttributesObject() {
        let attributes = super.createAttributesObject();

        // A `type` *state variable*, not just the attribute: subclasses declare
        // attributes typed `_componentListWithSelectableType`, whose own `type`
        // resolves through a `parentStateVariable` of this name. Without it
        // those attributes silently fall back to a numeric reading. See #1825.
        attributes.type = {
            ...returnListTypeAttribute(),
            createStateVariable: "type",
            defaultValue: null,
        };

        // Not used by the composite itself; forwarded to each `<math>` it
        // creates.
        Object.assign(attributes, returnPassThroughAttributeDeclarations());

        attributes.asList = {
            createPrimitiveOfType: "boolean",
            createStateVariable: "asList",
            defaultValue: true,
            highlighted: true,
            description:
                "Whether to render the items separated by commas (true) or with no separator (false).",
        };

        return attributes;
    }

    // Include children that can be added due to sugar
    static additionalSchemaChildren = ["string"];

    static returnSugarInstructions() {
        let sugarInstructions = super.returnSugarInstructions();

        sugarInstructions.push(
            returnBreakStringsIntoTypeSugarInstruction(this.componentType),
        );

        return sugarInstructions;
    }

    static returnChildGroups() {
        return [
            {
                group: "anything",
                componentTypes: ["_base"],
            },
        ];
    }

    static returnStateVariableDefinitions() {
        let stateVariableDefinitions = super.returnStateVariableDefinitions();

        Object.assign(
            stateVariableDefinitions,
            returnListValueStateVariableDefinitions({
                componentName: this.componentType,
                supportProps: false,
            }),
        );

        // Overridden by subclasses. Receives `{ values, numeric }` and returns
        // `{ counts, labels, diagnostics }`, the last two optional.
        stateVariableDefinitions.countValues = {
            returnDependencies: () => ({}),
            definition: () => ({
                setValue: {
                    countValues: () => ({ counts: [], labels: null }),
                },
            }),
        };

        stateVariableDefinitions.countingResults = {
            additionalStateVariablesDefined: ["countLabels"],
            returnDependencies: () => ({
                listValues: {
                    dependencyType: "stateVariable",
                    variableName: "listValues",
                },
                allAreNumeric: {
                    dependencyType: "stateVariable",
                    variableName: "allAreNumeric",
                },
                countValues: {
                    dependencyType: "stateVariable",
                    variableName: "countValues",
                },
            }),
            definition({ dependencyValues }) {
                const { counts, labels, diagnostics } =
                    dependencyValues.countValues({
                        values: dependencyValues.listValues,
                        numeric: dependencyValues.allAreNumeric,
                    });

                return {
                    setValue: {
                        countingResults: counts,
                        countLabels: labels ?? null,
                    },
                    sendDiagnostics: diagnostics ?? [],
                };
            },
        };

        stateVariableDefinitions.readyToExpandWhenResolved = {
            returnDependencies: () => ({
                countingResults: {
                    dependencyType: "stateVariable",
                    variableName: "countingResults",
                },
            }),
            // When this state variable is marked stale it indicates we should
            // update replacements. For this to work, we must get its value in
            // the replacement functions so that the variable is marked fresh.
            markStale: () => ({ updateReplacements: true }),
            definition: function () {
                return { setValue: { readyToExpandWhenResolved: true } };
            },
        };

        return stateVariableDefinitions;
    }

    static async createSerializedReplacements({
        component,
        componentInfoObjects,
        workspace,
        nComponents,
    }) {
        return createValueListReplacements({
            component,
            values: (await component.stateValues.countingResults).map((count) =>
                me.fromAst(count),
            ),
            componentType: "math",
            attributesToConvert: returnPassThroughAttributes(component),
            componentInfoObjects,
            workspace,
            nComponents,
        });
    }

    static async calculateReplacementChanges({
        component,
        componentInfoObjects,
        workspace,
        nComponents,
    }) {
        return calculateValueListReplacementChanges({
            component,
            values: (await component.stateValues.countingResults).map((count) =>
                me.fromAst(count),
            ),
            componentType: "math",
            attributesToConvert: returnPassThroughAttributes(component),
            componentInfoObjects,
            workspace,
            nComponents,
        });
    }

    addOwnPotentialRendererTypes(rendererTypes, visited) {
        super.addOwnPotentialRendererTypes(rendererTypes, visited);

        // The replacements are `<math>` components whatever the children are.
        addReplacementRendererType({
            component: this,
            componentType: "math",
            rendererTypes,
        });
    }
}
