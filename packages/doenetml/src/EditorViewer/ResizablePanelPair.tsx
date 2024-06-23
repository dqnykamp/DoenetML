import React, { useRef } from "react";
import { Center, Grid, GridItem, Icon } from "@chakra-ui/react";
import { BsGripHorizontal, BsGripVertical } from "react-icons/bs";
import { IconType } from "react-icons/lib";

export const ResizablePanelPair = ({
    panelA,
    panelB,
    preferredDirection = "horizontal",
    centerWidth = "10px",
    width = "100%",
    height = "100%",
}: {
    panelA: React.JSX.Element;
    panelB: React.JSX.Element;
    preferredDirection?: "horizontal" | "vertical";
    centerWidth?: string;
    width?: string;
    height?: string;
}) => {
    const wrapperRef = useRef<HTMLDivElement>(null);
    const handleClicked = useRef(false);
    const handleDragged = useRef(false);

    const direction = useRef(preferredDirection);

    const onMouseDown = (
        event: React.MouseEvent<HTMLDivElement, MouseEvent>,
    ) => {
        event.preventDefault();
        handleClicked.current = true;
    };

    const onMouseMove = (
        event: React.MouseEvent<HTMLDivElement, MouseEvent>,
    ) => {
        //TODO: minimum movement calc
        if (handleClicked.current) {
            event.preventDefault();
            handleDragged.current = true;

            if (direction.current === "vertical") {
                let proportion =
                    (event.clientY - wrapperRef.current!.offsetTop) /
                    wrapperRef.current!.clientHeight;

                //using a ref to save without react refresh
                wrapperRef.current!.style.gridTemplateRows = `${proportion}fr ${centerWidth} ${
                    1 - proportion
                }fr`;
            } else {
                let proportion =
                    (event.clientX - wrapperRef.current!.offsetLeft) /
                    wrapperRef.current!.clientWidth;

                //using a ref to save without react refresh
                wrapperRef.current!.style.gridTemplateColumns = `${proportion}fr ${centerWidth} ${
                    1 - proportion
                }fr`;
            }
        }
    };

    const onMouseUp = () => {
        if (handleClicked.current) {
            handleClicked.current = false;
            if (handleDragged.current) {
                handleDragged.current = false;
            }
        }
    };

    let templateAreas: string,
        gridTemplateRows: string,
        gridTemplateColumns: string,
        gutterHeight: string,
        gutterWidth: string,
        gutterIcon: IconType,
        gutterCursor: string;

    if (direction.current === "vertical") {
        templateAreas = `"panelA"
                         "middleGutter"
                         "panelB"`;
        gridTemplateRows = `1fr ${centerWidth} 1fr`;
        gridTemplateColumns = `1fr`;
        gutterHeight = centerWidth;
        gutterWidth = "100%";
        gutterIcon = BsGripHorizontal;
        gutterCursor = "row-resize";
    } else {
        templateAreas = `"panelA middleGutter panelB"`;
        gridTemplateRows = `1fr`;
        gridTemplateColumns = `.5fr ${centerWidth} .5fr`;
        gutterHeight = "100%";
        gutterWidth = centerWidth;
        gutterIcon = BsGripVertical;
        gutterCursor = "col-resize";
    }

    return (
        <Grid
            width={width}
            height={height}
            templateAreas={templateAreas}
            gridTemplateRows={gridTemplateRows}
            gridTemplateColumns={gridTemplateColumns}
            overflow="hidden"
            onMouseUp={onMouseUp}
            onMouseMove={onMouseMove}
            onMouseLeave={onMouseUp}
            ref={wrapperRef}
        >
            <GridItem
                area="panelA"
                width="100%"
                height="100%"
                placeSelf="center"
                overflow="hidden"
            >
                {panelA}
            </GridItem>
            <GridItem
                area="middleGutter"
                width="100%"
                height="100%"
                placeSelf="center"
            >
                <Center
                    cursor={gutterCursor}
                    background="doenet.mainGray"
                    boxSizing="border-box"
                    border="solid 1px"
                    borderColor="doenet.mediumGray"
                    height={gutterHeight}
                    width={gutterWidth}
                    onMouseDown={onMouseDown}
                    data-test="contentPanelDragHandle"
                >
                    <Icon ml="0" as={gutterIcon} />
                </Center>
            </GridItem>
            <GridItem
                area="panelB"
                width="100%"
                height="100%"
                placeSelf="center"
                overflow="hidden"
            >
                {panelB}
            </GridItem>
        </Grid>
    );
};
