import { useMutation, useQuery } from "@apollo/client";
import { toaster } from "evergreen-ui";
import { useFormik } from "formik";
import { translate } from "piteo-kit";
import { useEffect, useState } from "react";
import { useParams } from "react-router-dom";
import {
  PropertyListQuery,
  PropertyUpdateMutation,
  PropertyUpdateMutationOptions,
} from "../../helpers";
import { useRouter } from "../../hooks/use-router";
import { Property, PropertyUpdateInput } from "../../types";
import { uncast } from "../../utils";
import { PropertyUpdateValidator } from "../../validators";
import { PropertyDialogProps } from "./property-dialog";

const _ = translate();

type WrappedComponentProps = unknown;

export const withContainer = (
  WrappedComponent,
): React.FunctionComponent<WrappedComponentProps> =>
  () => {
    const router = useRouter();
    const { id: propertyId } = useParams();
    const [property, setProperty] = useState(null);
    const [isShown, setIsShown] = useState(true);

    const { data: { properties } = { properties: null } } = useQuery(
      PropertyListQuery,
      {
        variables: { id: propertyId },
      },
    );

    useEffect(() => {
      setProperty(properties?.find((item: Property) => item.id === propertyId));
    }, [properties, propertyId]);

    const [propertyUpdate] = useMutation(
      PropertyUpdateMutation,
      PropertyUpdateMutationOptions(),
    );

    const initialValues = uncast(property);

    const validationSchema = PropertyUpdateValidator;

    const handleSubmit = async (values: PropertyUpdateInput): Promise<void> => {
      // keep only form values that are known input fields
      const input = validationSchema.cast(values, { stripUnknown: true });
      try {
        await propertyUpdate({ variables: { input } });
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
      initialValues,
      validationSchema,
      onSubmit: handleSubmit,
    });

    const componentProps: PropertyDialogProps = {
      form,
      validationSchema,
      isShown,
      onCloseComplete: handleCloseComplete,
      title: _("edit_property"),
    };

    return WrappedComponent(componentProps);
  };
