import { useCallback, useState } from "react";
import CodeMirror from "@uiw/react-codemirror";
import { nord } from "@uiw/codemirror-theme-nord";
import { interprete } from "../../pkg";
import { icelang } from "../utils/language";
import { FaPlay, FaDesktop } from "react-icons/fa";

const Playground = () => {
  const [textValue, setTextValue] = useState("");
  const [errorMessage, setErrorMessage] = useState("");

  const onChange = useCallback((value: string) => {
    setTextValue(value);
  }, []);

  const runCode = () => {
    clearOutupt();
    const error = interprete(textValue);

    if (error) {
      setErrorMessage(error);
    }
  };

  const clearOutupt = () => {
    setErrorMessage("");
    const output = document.getElementById("output") as HTMLElement;
    output.querySelectorAll("div").forEach((child) => {
      child.remove();
    });
  };

  return (
    <div className="p-2 w-full flex flex-col gap-3 items-center justify-center">
      <div className="max-w-[700px] h-[450px] w-full overflow-scroll rounded-md relative">
        <CodeMirror
          value={textValue}
          height="450px"
          theme={nord}
          extensions={[icelang()]}
          onChange={onChange}
          className="rounded-md text-[0.9rem]"
        />
        <button
          className="mt-6 px-6 py-3 absolute bottom-2 right-2 flex items-center rounded-md font-semibold text-nord-1 bg-nord-8 hover:bg-nord-4 duration-300"
          onClick={() => runCode()}
        >
          <FaPlay className="mr-2" size={12} /> Run
        </button>
      </div>
      <div className="flex items-center font-bold text-nord-5 text-[1.1rem]">
        <span>Output</span>
        <FaDesktop className="ml-2" />
      </div>
      <div
        id="output"
        className="font-mono px-6 py-4 w-full max-w-[700px] min-h-[300px] max-h-[300px] overflow-scroll rounded-md text-nord-5 bg-nord-0"
      ></div>
    </div>
  );
};

export default Playground;
