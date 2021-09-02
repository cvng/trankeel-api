import React from "react";
import { FactoryHelper } from "../../helpers/factory-helper";
import { Themable } from "../common/themable";
import { TenantDocumentList } from "./tenant-document-list";

export default {
  title: "Tenant/TenantDocumentList",
  component: TenantDocumentList,
};

const fileList = FactoryHelper.fileList();

export const loading = () =>
  <Themable>
    <TenantDocumentList loading />
  </Themable>;

export const noData = () =>
  <Themable>
    <TenantDocumentList files={[]} />;
  </Themable>;

export const filteredWithNoData = () =>
  <Themable>
    <TenantDocumentList files={[]} filterEnabled />
  </Themable>;

export const withData = () =>
  <Themable>
    <TenantDocumentList files={fileList} />
  </Themable>;
