import { parser } from "./icelang";
import {
  LRLanguage,
  LanguageSupport,
  delimitedIndent,
  indentNodeProp,
} from "@codemirror/language";
import { completeFromList } from "@codemirror/autocomplete";
import { styleTags, tags } from "@lezer/highlight";

const parserWithMetadata = parser.configure({
  props: [
    indentNodeProp.add({
      Block: delimitedIndent({ closing: "}", align: false }),
    }),
    styleTags({
      Identifier: tags.variableName,
      BooleanLiteral: tags.bool,
      String: tags.string,
      LineComment: tags.lineComment,
      "if else for in while loop match lambda to": tags.keyword,
      "set function": tags.definitionKeyword,
      "return continue break": tags.controlKeyword,
      Number: tags.number,
      "( )": tags.paren,
    }),
  ],
});

const icelangLanguage = LRLanguage.define({
  parser: parserWithMetadata,
});

const iceCompletion = icelangLanguage.data.of({
  autocomplete: completeFromList([
    { label: "set", type: "keyword" },
    { label: "function", type: "keyword" },
    { label: "lambda", type: "keyword" },
    { label: "if", type: "keyword" },
    { label: "else", type: "keyword" },
    { label: "for", type: "keyword" },
    { label: "in", type: "keyword" },
    { label: "to", type: "keyword" },
    { label: "while", type: "keyword" },
    { label: "loop", type: "keyword" },
    { label: "match", type: "keyword" },
    { label: "continue", type: "keyword" },
    { label: "break", type: "keyword" },
    { label: "return", type: "keyword" },
    { label: "print", type: "function" },
    { label: "true", type: "keyword" },
    { label: "false", type: "keyword" },
    {
      label: "length",
      type: "function",
      info: "length(data: string | array | object): number",
    },
    {
      label: "parse_number",
      type: "function",
      info: "parse_number(string: any): number",
    },
    {
      label: "type_of",
      type: "function",
      info: "type_of(data: any): string",
    },
    {
      label: "print",
      type: "function",
      info: "print(data: ...any): null",
    },
    { label: "sqrt", type: "function", info: "sqrt(x: number): number" },
    {
      label: "pow",
      type: "function",
      info: "pow(x: number, y: number): number",
    },
    { label: "floor", type: "function", info: "floor(x: number): number" },
    { label: "round", type: "function", info: "round(x: number): number" },
    { label: "ceil", type: "function", info: "ceil(x: number): number" },
  ]),
});

export const icelang = () => {
  return new LanguageSupport(icelangLanguage, [iceCompletion]);
};
