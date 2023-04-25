import { nord } from "@uiw/codemirror-theme-nord";
import ReactCodeMirror from "@uiw/react-codemirror";
import { icelang } from "../utils/language";

type props = {
  value: string;
};

const CodeBlock: React.FC<props> = ({ value }) => {
  return (
    <div className="rounded-md overflow-hidden">
      <ReactCodeMirror
        value={value}
        theme={nord}
        extensions={[icelang()]}
        editable={false}
        basicSetup={{
          highlightActiveLine: false,
          highlightActiveLineGutter: false,
        }}
        className="text-[0.9rem]"
      />
    </div>
  );
};

export default CodeBlock;
