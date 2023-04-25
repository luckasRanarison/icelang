import { useCallback, useRef, useState } from "react";
import CodeMirror from "@uiw/react-codemirror";
import { nord } from "@uiw/codemirror-theme-nord";
import { interprete } from "../../pkg";
import { icelang } from "../utils/language";
import {
  FaPlay,
  FaDesktop,
  FaCheckCircle,
  FaTimesCircle,
} from "react-icons/fa";

const Playground = () => {
  const [textValue, setTextValue] = useState('print("Hello World")');
  const [errorMessage, setErrorMessage] = useState("");
  const outputRef = useRef<HTMLDivElement>(null);

  const onChange = useCallback((value: string) => {
    setTextValue(value);
  }, []);

  const runCode = () => {
    clearOutupt();
    const error = interprete(textValue);

    if (error) {
      setErrorMessage(error);
    }

    if (outputRef.current) {
      const outputY = outputRef.current.offsetTop;
      window.scrollBy({ top: outputY + 260, behavior: "smooth" });
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
    <div className="p-4 w-full flex flex-col items-center justify-center">
      <div id="playground" className="pt-4 mb-10 w-full max-w-[700px]">
        <div className="mb-2 text-title font-semibold text-nord-6">
          Playground
        </div>
        <p className="text-nord-4 leading-7">
          Icelang coding playground powered by Web Assembly. I/O functions like
          import/export and readline won't work in the playground, only{" "}
          <span className="text-nord-8">print</span> can be used with the fake
          output.
        </p>
      </div>
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
      <div className="mt-4 px-6 py-4 w-full max-w-[700px] min-h-[300px] max-h-[300px] relative flex flex-col rounded-md text-nord-5 bg-nord-0">
        <div className="flex justify-between items-center">
          <div className="flex items-center font-bold text-[1.1rem]">
            <span>Output</span>
            <FaDesktop className="ml-2" />
          </div>
          <div
            className={`flex items-center ${
              errorMessage ? "text-nord-11" : "text-nord-14"
            }`}
          >
            {errorMessage ? <FaTimesCircle /> : <FaCheckCircle />}
            <span className="ml-2 font-bold">
              {errorMessage ? "Error" : "Success"}
            </span>
          </div>
        </div>
        <div
          id="output"
          ref={outputRef}
          className="mt-4 h-full w-full overflow-scroll"
        ></div>
        <div className="font-bold text-nord-11">{errorMessage}</div>
      </div>
    </div>
  );
};

export default Playground;
