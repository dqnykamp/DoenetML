import React, { useEffect, useRef } from 'react';
import PageViewer from '../../../Viewer/PageViewer';
import { 
  useRecoilValue, 
  atom,
  useRecoilCallback,
  useRecoilState,
  useSetRecoilState,
} from 'recoil';
import { searchParamAtomFamily } from '../NewToolRoot';
import { 
  itemHistoryAtom, 
  fileByContentId, 
  variantInfoAtom, 
  variantPanelAtom,
 } from '../ToolHandlers/CourseToolHandler';
//  import { currentDraftSelectedAtom } from '../Menus/VersionHistory'

export const viewerDoenetMLAtom = atom({
  key:"viewerDoenetMLAtom",
  default:""
})

export const textEditorDoenetMLAtom = atom({
  key:"textEditorDoenetMLAtom",
  default:""
})

export const updateTextEditorDoenetMLAtom = atom({
  key:"updateTextEditorDoenetMLAtom",
  default:""
})

//Boolean initialized editor tool start up
export const editorDoenetIdInitAtom = atom({
  key:"editorDoenetIdInitAtom",
  default:""
})

export const refreshNumberAtom = atom({
  key:"refreshNumberAtom",
  default:0
})

export const editorViewerErrorStateAtom = atom({
  key:"editorViewerErrorStateAtom",
  default:false
})

export default function EditorViewer(){
  // let refreshCount = useRef(1);
  // console.log(">>>>===EditorViewer",refreshCount.current)
  // refreshCount.current++;
  const viewerDoenetML = useRecoilValue(viewerDoenetMLAtom);
  const paramDoenetId = useRecoilValue(searchParamAtomFamily('doenetId')) 
  const initilizedDoenetId = useRecoilValue(editorDoenetIdInitAtom);
  const [variantInfo,setVariantInfo] = useRecoilState(variantInfoAtom);
  const setVariantPanel = useSetRecoilState(variantPanelAtom);
  const setEditorInit = useSetRecoilState(editorDoenetIdInitAtom);
  const refreshNumber = useRecoilValue(refreshNumberAtom);
  const setIsInErrorState = useSetRecoilState(editorViewerErrorStateAtom);


  let initDoenetML = useRecoilCallback(({snapshot,set})=> async (doenetId)=>{
    const versionHistory = await snapshot.getPromise((itemHistoryAtom(doenetId)));
    const cid = versionHistory.draft.cid;
    let response = await snapshot.getPromise(fileByContentId(cid));
    if (typeof response === "object"){
      response = response.data;
    }
    const doenetML = response;

    set(updateTextEditorDoenetMLAtom,doenetML);
    set(textEditorDoenetMLAtom,doenetML)
    set(viewerDoenetMLAtom,doenetML)
    set(editorDoenetIdInitAtom,doenetId);
  },[])


  useEffect(() => {
      if (paramDoenetId !== ''){
        initDoenetML(paramDoenetId)
      }
    return () => {
      setEditorInit("");
    }
  }, [paramDoenetId]);

  if (paramDoenetId !== initilizedDoenetId){
    //DoenetML is changing to another DoenetID
    return null;
  }


  let attemptNumber = 1;
  let solutionDisplayMode = "button";

  if (variantInfo.lastUpdatedIndexOrName === 'Index'){
    setVariantInfo((was)=>{
      let newObj = {...was}; 
      newObj.lastUpdatedIndexOrName = null; 
      newObj.requestedVariant = {index:variantInfo.index};
    return newObj})

  }else if (variantInfo.lastUpdatedIndexOrName === 'Name'){
    setVariantInfo((was)=>{
      let newObj = {...was}; 
      newObj.lastUpdatedIndexOrName = null; 
      newObj.requestedVariant = {name:variantInfo.name};
    return newObj})

  }



  function variantCallback(generatedVariantInfo, allPossibleVariants){
    // console.log(">>>variantCallback",generatedVariantInfo,allPossibleVariants)
    const cleanGeneratedVariant = JSON.parse(JSON.stringify(generatedVariantInfo))
    cleanGeneratedVariant.lastUpdatedIndexOrName = null 
    setVariantPanel({
      index:cleanGeneratedVariant.index,
      name:cleanGeneratedVariant.name,
      allPossibleVariants
    });
    setVariantInfo((was)=>{
      let newObj = {...was}
      Object.assign(newObj,cleanGeneratedVariant)
      return newObj;
    });
  }



  console.log(`>>>>Show PageViewer with value -${viewerDoenetML}- -${refreshNumber}-`)
  // console.log(`>>>> refreshNumber -${refreshNumber}-`)
  // console.log(`>>>> attemptNumber -${attemptNumber}-`)
  // console.log('>>>PageViewer Read Only:',!isCurrentDraft)
  // console.log('>>>>variantInfo.requestedVariant',variantInfo.requestedVariant)

  return <PageViewer
    key={`pageViewer${refreshNumber}`}
    doenetML={viewerDoenetML}
    flags={{
      showCorrectness: true,
      readOnly: false,
      solutionDisplayMode: solutionDisplayMode,
      showFeedback: true,
      showHints: true,
      allowLoadState: false,
      allowSaveState: false,
      allowLocalState: false,
      allowSaveSubmissions: false,
      allowSaveEvents: false
    }}
    attemptNumber={attemptNumber}
    generatedVariantCallback={variantCallback} //TODO:Replace
    requestedVariant={variantInfo.requestedVariant}
    setIsInErrorState={setIsInErrorState}
    pageIsActive={true}
    /> 
}


