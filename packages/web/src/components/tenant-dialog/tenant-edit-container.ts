import { useMutation, useQuery } from "@apollo/client";
import { toaster } from "evergreen-ui";
import { useFormik } from "formik";
import { translate } from "piteo-kit";
import { useEffect, useState } from "react";
import { useParams } from "react-router-dom";
import {
  TenantListQuery,
  TenantUpdateMutation,
  TenantUpdateMutationOptions,
} from "../../helpers";
import { useRouter } from "../../hooks/use-router";
import { Tenant, TenantInput } from "../../types";
import { uncast } from "../../utils";
import { TenantValidator } from "../../validators";
import { TenantDialogProps } from "./tenant-dialog";

const _ = translate();

type WrappedComponentProps = unknown;

export const withContainer = (
  WrappedComponent,
): React.FunctionComponent<WrappedComponentProps> =>
  () => {
    const router = useRouter();
    const { id: tenantId } = useParams();
    const [tenant, setTenant] = useState(null);
    const [isShown, setIsShown] = useState(true);

    const { data: { tenants } = { tenants: null } } = useQuery(
      TenantListQuery,
      {
        variables: { id: tenantId },
      },
    );

    useEffect(() => {
      setTenant(tenants?.find((item: Tenant) => item.id === tenantId));
    }, [tenants, tenantId]);

    const validationSchema = TenantValidator;

    const [tenantUpdate] = useMutation(
      TenantUpdateMutation,
      TenantUpdateMutationOptions(),
    );

    const handleSubmit = async (values: TenantInput): Promise<void> => {
      // keep only form values that are known input fields
      const input = validationSchema.cast(values, { stripUnknown: true });
      try {
        await tenantUpdate({ variables: { input } });
        setIsShown(false);
      } catch {
        toaster.danger(_("error_smi"));
      }
    };

    const handleCloseComplete = (): void => {
      router.goBack();
    };

    const form = useFormik({
      enableReinitialize: true,
      initialValues: uncast(tenant),
      validationSchema,
      onSubmit: handleSubmit,
    });

    const componentProps: TenantDialogProps = {
      form,
      validationSchema,
      isShown,
      onCloseComplete: handleCloseComplete,
      title: _("edit_tenant"),
    };

    return WrappedComponent(componentProps);
  };
