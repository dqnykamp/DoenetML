import me from "math-expressions";
import { cesc } from "../../../src/utils/url";

describe("Equilibriumline Tag Tests", function () {
  beforeEach(() => {
    cy.clearIndexedDB();
    cy.visit("/src/Tools/cypressTest/");
  });

  it("equilibriumline change stable", () => {
    cy.window().then(async (win) => {
      win.postMessage(
        {
          doenetML: `
    <text>a</text>
    <graph name="g" newNamespace>
      <equilibriumline name="A" switchAble>y=4</equilibriumline>
      <equilibriumline name="B" stable="false">y=7</equilibriumline>
      <equilibriumline name="C" stable="$(../b1)" styleNumber="2">y=-9</equilibriumline>
      <equilibriumline name="D" stable="$(../b2)" styleNumber="2" switchable>y=-3</equilibriumline>
    </graph>
  
    <booleaninput name="b1" />
    <booleaninput name="b2" />

    <p><aslist>
    $(g/A.stable{assignNames="gAs"})
    $(g/B.stable{assignNames="gBs"})
    $(g/C.stable{assignNames="gCs"})
    $(g/D.stable{assignNames="gDs"})
    </aslist>
    </p>

    $g{name="g2"}

    <p><aslist>
    $(g2/A.stable{assignNames="g2As"})
    $(g2/B.stable{assignNames="g2Bs"})
    $(g2/C.stable{assignNames="g2Cs"})
    $(g2/D.stable{assignNames="g2Ds"})
    </aslist>
    </p>
    `,
        },
        "*",
      );
    });

    cy.get(cesc("#\\/_text1")).should("have.text", "a"); // to wait until loaded

    cy.get(cesc("#\\/gAs")).should("have.text", "true");
    cy.get(cesc("#\\/gBs")).should("have.text", "false");
    cy.get(cesc("#\\/gCs")).should("have.text", "false");
    cy.get(cesc("#\\/gDs")).should("have.text", "false");
    cy.get(cesc("#\\/g2As")).should("have.text", "true");
    cy.get(cesc("#\\/g2Bs")).should("have.text", "false");
    cy.get(cesc("#\\/g2Cs")).should("have.text", "false");
    cy.get(cesc("#\\/g2Ds")).should("have.text", "false");

    cy.window().then(async (win) => {
      let stateVariables = await win.returnAllStateVariables1();

      expect(stateVariables["/g/A"].stateValues.stable).eq(true);
      expect(stateVariables["/g/B"].stateValues.stable).eq(false);
      expect(stateVariables["/g/C"].stateValues.stable).eq(false);
      expect(stateVariables["/g/D"].stateValues.stable).eq(false);
      expect(stateVariables["/g/A"].stateValues.equation).eqls(["=", "y", 4]);
      expect(stateVariables["/g/B"].stateValues.equation).eqls(["=", "y", 7]);
      expect(stateVariables["/g/C"].stateValues.equation).eqls(["=", "y", -9]);
      expect(stateVariables["/g/D"].stateValues.equation).eqls(["=", "y", -3]);

      expect(stateVariables["/g2/A"].stateValues.stable).eq(true);
      expect(stateVariables["/g2/B"].stateValues.stable).eq(false);
      expect(stateVariables["/g2/C"].stateValues.stable).eq(false);
      expect(stateVariables["/g2/D"].stateValues.stable).eq(false);
      expect(stateVariables["/g2/A"].stateValues.equation).eqls(["=", "y", 4]);
      expect(stateVariables["/g2/B"].stateValues.equation).eqls(["=", "y", 7]);
      expect(stateVariables["/g2/C"].stateValues.equation).eqls(["=", "y", -9]);
      expect(stateVariables["/g2/D"].stateValues.equation).eqls(["=", "y", -3]);
    });

    cy.log("switch C via boolean input");
    cy.get(cesc("#\\/b1")).click();

    cy.get(cesc("#\\/gAs")).should("have.text", "true");
    cy.get(cesc("#\\/gBs")).should("have.text", "false");
    cy.get(cesc("#\\/gCs")).should("have.text", "true");
    cy.get(cesc("#\\/gDs")).should("have.text", "false");
    cy.get(cesc("#\\/g2As")).should("have.text", "true");
    cy.get(cesc("#\\/g2Bs")).should("have.text", "false");
    cy.get(cesc("#\\/g2Cs")).should("have.text", "true");
    cy.get(cesc("#\\/g2Ds")).should("have.text", "false");

    cy.window().then(async (win) => {
      let stateVariables = await win.returnAllStateVariables1();

      expect(stateVariables["/g/A"].stateValues.stable).eq(true);
      expect(stateVariables["/g/B"].stateValues.stable).eq(false);
      expect(stateVariables["/g/C"].stateValues.stable).eq(true);
      expect(stateVariables["/g/D"].stateValues.stable).eq(false);

      expect(stateVariables["/g2/A"].stateValues.stable).eq(true);
      expect(stateVariables["/g2/B"].stateValues.stable).eq(false);
      expect(stateVariables["/g2/C"].stateValues.stable).eq(true);
      expect(stateVariables["/g2/D"].stateValues.stable).eq(false);
    });

    cy.log("switch D via boolean input");
    cy.get(cesc("#\\/b2")).click();

    cy.get(cesc("#\\/gAs")).should("have.text", "true");
    cy.get(cesc("#\\/gBs")).should("have.text", "false");
    cy.get(cesc("#\\/gCs")).should("have.text", "true");
    cy.get(cesc("#\\/gDs")).should("have.text", "true");
    cy.get(cesc("#\\/g2As")).should("have.text", "true");
    cy.get(cesc("#\\/g2Bs")).should("have.text", "false");
    cy.get(cesc("#\\/g2Cs")).should("have.text", "true");
    cy.get(cesc("#\\/g2Ds")).should("have.text", "true");

    cy.window().then(async (win) => {
      let stateVariables = await win.returnAllStateVariables1();

      expect(stateVariables["/g/A"].stateValues.stable).eq(true);
      expect(stateVariables["/g/B"].stateValues.stable).eq(false);
      expect(stateVariables["/g/C"].stateValues.stable).eq(true);
      expect(stateVariables["/g/D"].stateValues.stable).eq(true);

      expect(stateVariables["/g2/A"].stateValues.stable).eq(true);
      expect(stateVariables["/g2/B"].stateValues.stable).eq(false);
      expect(stateVariables["/g2/C"].stateValues.stable).eq(true);
      expect(stateVariables["/g2/D"].stateValues.stable).eq(true);
    });

    cy.log("switch A via first action");
    cy.window().then(async (win) => {
      await win.callAction1({
        actionName: "switchLine",
        componentName: "/g/A",
      });

      let stateVariables = await win.returnAllStateVariables1();

      expect(stateVariables["/g/A"].stateValues.stable).eq(false);
      expect(stateVariables["/g/B"].stateValues.stable).eq(false);
      expect(stateVariables["/g/C"].stateValues.stable).eq(true);
      expect(stateVariables["/g/D"].stateValues.stable).eq(true);

      expect(stateVariables["/g2/A"].stateValues.stable).eq(false);
      expect(stateVariables["/g2/B"].stateValues.stable).eq(false);
      expect(stateVariables["/g2/C"].stateValues.stable).eq(true);
      expect(stateVariables["/g2/D"].stateValues.stable).eq(true);

      cy.get(cesc("#\\/gAs")).should("have.text", "false");
      cy.get(cesc("#\\/gBs")).should("have.text", "false");
      cy.get(cesc("#\\/gCs")).should("have.text", "true");
      cy.get(cesc("#\\/gDs")).should("have.text", "true");
      cy.get(cesc("#\\/g2As")).should("have.text", "false");
      cy.get(cesc("#\\/g2Bs")).should("have.text", "false");
      cy.get(cesc("#\\/g2Cs")).should("have.text", "true");
      cy.get(cesc("#\\/g2Ds")).should("have.text", "true");
    });

    cy.log("switch A via second action");
    cy.window().then(async (win) => {
      await win.callAction1({
        actionName: "switchLine",
        componentName: "/g2/A",
      });

      let stateVariables = await win.returnAllStateVariables1();

      expect(stateVariables["/g/A"].stateValues.stable).eq(true);
      expect(stateVariables["/g/B"].stateValues.stable).eq(false);
      expect(stateVariables["/g/C"].stateValues.stable).eq(true);
      expect(stateVariables["/g/D"].stateValues.stable).eq(true);

      expect(stateVariables["/g2/A"].stateValues.stable).eq(true);
      expect(stateVariables["/g2/B"].stateValues.stable).eq(false);
      expect(stateVariables["/g2/C"].stateValues.stable).eq(true);
      expect(stateVariables["/g2/D"].stateValues.stable).eq(true);

      cy.get(cesc("#\\/gAs")).should("have.text", "true");
      cy.get(cesc("#\\/gBs")).should("have.text", "false");
      cy.get(cesc("#\\/gCs")).should("have.text", "true");
      cy.get(cesc("#\\/gDs")).should("have.text", "true");
      cy.get(cesc("#\\/g2As")).should("have.text", "true");
      cy.get(cesc("#\\/g2Bs")).should("have.text", "false");
      cy.get(cesc("#\\/g2Cs")).should("have.text", "true");
      cy.get(cesc("#\\/g2Ds")).should("have.text", "true");
    });

    cy.log("cannot switch B via action");
    cy.window().then(async (win) => {
      await win.callAction1({
        actionName: "switchLine",
        componentName: "/g/B",
      });

      let stateVariables = await win.returnAllStateVariables1();

      expect(stateVariables["/g/A"].stateValues.stable).eq(true);
      expect(stateVariables["/g/B"].stateValues.stable).eq(false);
      expect(stateVariables["/g/C"].stateValues.stable).eq(true);
      expect(stateVariables["/g/D"].stateValues.stable).eq(true);

      expect(stateVariables["/g2/A"].stateValues.stable).eq(true);
      expect(stateVariables["/g2/B"].stateValues.stable).eq(false);
      expect(stateVariables["/g2/C"].stateValues.stable).eq(true);
      expect(stateVariables["/g2/D"].stateValues.stable).eq(true);

      cy.get(cesc("#\\/gAs")).should("have.text", "true");
      cy.get(cesc("#\\/gBs")).should("have.text", "false");
      cy.get(cesc("#\\/gCs")).should("have.text", "true");
      cy.get(cesc("#\\/gDs")).should("have.text", "true");
      cy.get(cesc("#\\/g2As")).should("have.text", "true");
      cy.get(cesc("#\\/g2Bs")).should("have.text", "false");
      cy.get(cesc("#\\/g2Cs")).should("have.text", "true");
      cy.get(cesc("#\\/g2Ds")).should("have.text", "true");
    });

    cy.log("cannot switch C via second action");
    cy.window().then(async (win) => {
      await win.callAction1({
        actionName: "switchLine",
        componentName: "/g2/C",
      });

      let stateVariables = await win.returnAllStateVariables1();

      expect(stateVariables["/g/A"].stateValues.stable).eq(true);
      expect(stateVariables["/g/B"].stateValues.stable).eq(false);
      expect(stateVariables["/g/C"].stateValues.stable).eq(true);
      expect(stateVariables["/g/D"].stateValues.stable).eq(true);

      expect(stateVariables["/g2/A"].stateValues.stable).eq(true);
      expect(stateVariables["/g2/B"].stateValues.stable).eq(false);
      expect(stateVariables["/g2/C"].stateValues.stable).eq(true);
      expect(stateVariables["/g2/D"].stateValues.stable).eq(true);

      cy.get(cesc("#\\/gAs")).should("have.text", "true");
      cy.get(cesc("#\\/gBs")).should("have.text", "false");
      cy.get(cesc("#\\/gCs")).should("have.text", "true");
      cy.get(cesc("#\\/gDs")).should("have.text", "true");
      cy.get(cesc("#\\/g2As")).should("have.text", "true");
      cy.get(cesc("#\\/g2Bs")).should("have.text", "false");
      cy.get(cesc("#\\/g2Cs")).should("have.text", "true");
      cy.get(cesc("#\\/g2Ds")).should("have.text", "true");
    });

    cy.log("switch D via second action");
    cy.window().then(async (win) => {
      await win.callAction1({
        actionName: "switchLine",
        componentName: "/g2/D",
      });

      let stateVariables = await win.returnAllStateVariables1();

      expect(stateVariables["/g/A"].stateValues.stable).eq(true);
      expect(stateVariables["/g/B"].stateValues.stable).eq(false);
      expect(stateVariables["/g/C"].stateValues.stable).eq(true);
      expect(stateVariables["/g/D"].stateValues.stable).eq(false);

      expect(stateVariables["/g2/A"].stateValues.stable).eq(true);
      expect(stateVariables["/g2/B"].stateValues.stable).eq(false);
      expect(stateVariables["/g2/C"].stateValues.stable).eq(true);
      expect(stateVariables["/g2/D"].stateValues.stable).eq(false);

      cy.get(cesc("#\\/gAs")).should("have.text", "true");
      cy.get(cesc("#\\/gBs")).should("have.text", "false");
      cy.get(cesc("#\\/gCs")).should("have.text", "true");
      cy.get(cesc("#\\/gDs")).should("have.text", "false");
      cy.get(cesc("#\\/g2As")).should("have.text", "true");
      cy.get(cesc("#\\/g2Bs")).should("have.text", "false");
      cy.get(cesc("#\\/g2Cs")).should("have.text", "true");
      cy.get(cesc("#\\/g2Ds")).should("have.text", "false");
    });
  });
});
