export {};
// import { Text } from "evergreen-ui";
// import React, { useState } from "react";
// import { Document, DocumentProps } from "./document";

// export default {
//   title: "Design System/Document",
//   component: Document,
// };

// const withContainer = (
//   WrappedComponent,
//   delegate,
// ): React.FunctionComponent =>
//   () => {
//     const [selectedIndex, setSelectedIndex] = useState(0);

//     const componentProps: DocumentProps = {
//       selectedIndex,
//       pages: [
//         <Text>Page 1</Text>,
//         <Text>Page 2</Text>,
//         <Text>Page 3</Text>,
//       ],
//       onClickPreviousPage: () => {
//         setSelectedIndex(selectedIndex - 1);
//         window.scrollTo(0, 0);
//       },
//       onClickNextPage: () => {
//         setSelectedIndex(selectedIndex + 1);
//         window.scrollTo(0, 0);
//       },
//       width: 400,
//       minHeight: 700,
//     };

//     return WrappedComponent(componentProps);
//   };

// FIXME: Wrapped the component with HOC cause error : Cannot read property 'largest' of undefined
// export const standard = () => {
//   return (<Themable>
//     {withContainer(Document, null)({})}
//   </Themable>);
// };
