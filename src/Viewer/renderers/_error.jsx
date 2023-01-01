import React from 'react';
import useDoenetRenderer from './useDoenetRenderer';

export default React.memo(function Error(props) {
  let { name, SVs } = useDoenetRenderer(props);

  let errorStyle = { backgroundColor: "#ff9999", textAlign: "center", borderWidth: 3, borderStyle: "solid" };

  return <div id={name} style={errorStyle}>Error: {SVs.message}</div>
})
