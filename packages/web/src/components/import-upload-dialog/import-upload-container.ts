import { useMutation } from "@apollo/client";
import { toaster } from "evergreen-ui";
import { useFormik } from "formik";
import { translate } from "piteo-kit";
import { FunctionComponent, useMemo, useState } from "react";
import {
  ImportUploadMutation,
  ImportUploadMutationOptions,
} from "../../helpers";
import { useRouter } from "../../hooks/use-router";
import { ImportInput } from "../../types";
import { code } from "../../utils";
import { ImportValidator } from "../../validators";
import { ImportUploadDialogProps } from "./import-upload-dialog";
import { parseWorkbook, Workbook } from "./parse-workbook";

const _ = translate();

export const withContainer = (Component): FunctionComponent<unknown> =>
  () => {
    const router = useRouter();

    const [isShown] = useState<boolean>(true);
    const [workbook, setWorkbook] = useState<Workbook>({
      properties: 0,
      tenants: 0,
      contracts: 0,
    });

    const [importUpload] = useMutation(
      ImportUploadMutation,
      ImportUploadMutationOptions(),
    );

    const validationSchema = ImportValidator;
    const initialValues = validationSchema.default();

    const handleSubmit = async (values: ImportInput): Promise<void> => {
      try {
        const input = validationSchema.cast(values, { stripUnknown: true });

        // Run async mutation without await
        await importUpload({ variables: { input } });

        toaster.success(_("import_upload_success"), {
          description: _("import_status_text"),
        });
        router.showPropertySynthesis();
      } catch (error) {
        toaster.danger(_(code(error)));
      }
    };

    const handleCloseComplete = (): void => {
      router.goBack();
    };

    const form = useFormik<ImportInput>({
      initialValues,
      validationSchema,
      onSubmit: handleSubmit,
    });

    const handleFileChange = (): void => {
      // @ts-ignore: https://github.com/piteo-team/piteo/pull/166
      parseWorkbook(form.values.files, setWorkbook);
    };

    useMemo(handleFileChange, [form.values.files]);

    const props: ImportUploadDialogProps = {
      form,
      validationSchema,
      isShown,
      workbook,
      onCloseComplete: handleCloseComplete,
    };

    return Component(props);
  };
