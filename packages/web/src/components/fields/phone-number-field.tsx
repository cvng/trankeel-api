import {
  Button,
  FormField,
  FormFieldProps,
  minorScale,
  SegmentedControlOwnProps,
  SelectMenu,
  SelectMenuItem,
  TextInput,
  TextInputProps,
} from "evergreen-ui";
import React, { useState } from "react";
import { translate } from "piteo-kit";
import { splitFieldProps } from "./utils";

const _ = translate();

// https://en.wikipedia.org/wiki/E.164
const COUNTRY_CODES = [
  { "label": "🇫🇷 France (+33)", "value": "+33" },
  { "label": "🇬🇵 Guadeloupe (+590)", "value": "+590" },
];

interface UnknownProps {
  filterPlaceholder?: string;
}

export type PhoneNumberFieldProps =
  & FormFieldProps
  & TextInputProps
  & UnknownProps
  & SegmentedControlOwnProps;

export const PhoneNumberField = ({
  // TextInput props
  value,
  onChange,
  // SelectMenu props
  title = _("select_country_code"),
  filterPlaceholder = _("filter_placeholder"),
  options = COUNTRY_CODES,
  // Rest props are spread on the FormField
  ...props
}: PhoneNumberFieldProps) => {
  const id = `PhoneNumberField-${props.name}`;

  options = options.length ? options : COUNTRY_CODES;

  const [selected, setSelected] = useState<SelectMenuItem>(
    options[0] as SelectMenuItem,
  );

  const buttonText = selected.label.split(" ")?.[0];
  const castedValue = value as string;
  const inputValue: string = castedValue?.replace(
    selected.value.toString(),
    "",
  );

  const handleChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    if (!event.target.value.startsWith(selected.value.toString())) {
      event.target.value = selected.value + event.target.value;
    }
  };

  const handleSelect = (item: SelectMenuItem) => {
    return setSelected(item);
  };

  const { fieldProps, remainingProps } = splitFieldProps(props);

  return (
    <FormField labelFor={id} {...fieldProps}>
      <SelectMenu
        title={title}
        // @ts-ignore: https://github.com/piteo-team/piteo/pull/166
        options={options}
        selected={selected.value as string}
        onSelect={handleSelect}
        filterPlaceholder={filterPlaceholder}
      >
        <Button type="button">{buttonText}</Button>
      </SelectMenu>
      <TextInput
        id={id}
        marginLeft={minorScale(1)}
        {...remainingProps}
        value={inputValue}
        onChange={handleChange}
      />
    </FormField>
  );
};
