import { parser } from "./icelang";
import {
  LRLanguage,
  LanguageSupport,
  delimitedIndent,
  foldInside,
  foldNodeProp,
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
      info: "Returns the length of strings, array and object",
    },
    {
      label: "type_of",
      type: "function",
      info: "Returns the type of the argument",
    },
    { label: "print", type: "function" },
    { label: "sqrt", type: "function" },
    { label: "pow", type: "function" },
    { label: "floor", type: "function" },
    { label: "round", type: "function" },
    { label: "ceil", type: "function" },
  ]),
});

export const icelang = () => {
  return new LanguageSupport(icelangLanguage, [iceCompletion]);
};
