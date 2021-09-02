import { majorScale, Pane } from "evergreen-ui";
import React, { useEffect, useState } from "react";
import { ReactLiquid } from "react-liquid";

export type DocumentViewerProps = {
  templateURL: string;
  templateData: Record<string, string>;
};

export const DocumentViewer: React.FunctionComponent<
  DocumentViewerProps
> = ({
  templateURL,
  templateData,
}) => {
  const [template, setTemplate] = useState(null);

  useEffect(() => {
    function getTemplate() {
      fetch(templateURL)
        .then((res) => res.text())
        .then((data) => {
          setTemplate(data);
        });
    }
    getTemplate();
  });

  return (
    <Pane padding={majorScale(2)}>
      <ReactLiquid
        template={template}
        data={templateData}
        render={(renderedTemplate) => {
          return <span dangerouslySetInnerHTML={renderedTemplate} />;
        }}
      />
    </Pane>
  );
};
