import React from "react";
import { DocumentViewer } from "./document-viewer";

export default {
  title: "Document/DocumentViewer",
  component: DocumentViewer,
};

export const standard = () => (
  <DocumentViewer
    templateURL="/bail_meuble.html"
    templateData={{
      property_address_inline: "168 Avenue Jules Ferry, 16000 Angoulême",
    }}
  />
);
