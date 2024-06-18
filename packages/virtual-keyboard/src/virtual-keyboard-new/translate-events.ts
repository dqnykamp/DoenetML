import { KeyCommand } from "./keys";

/**
 * Translate a list of commands returned by the virtual keyboard into the form that Recoil expects.
 * These should either be sent to the callback returned by `useRecoilValue(focusedMathField)` or
 * `useRecoilValue(focusedMathFieldReturn)`.
 */
export function translateKeyboardEvent(
    commands: KeyCommand[],
    timestamp: number,
) {
    return commands.map((command) => {
        if (command.type === "keystroke" && command.command === "Enter") {
            return { focusedMathFieldReturn: "", timestamp };
        }

        return {
            focusedMathField: `${command.type} ${command.command}`,
            timestamp,
        };
    });
}
