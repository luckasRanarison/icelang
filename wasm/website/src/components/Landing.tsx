import { FaCode, FaCompass } from "react-icons/fa";

const Landing = () => {
  return (
    <div className="flex flex-col h-[80vh] items-center justify-center text-center text-nord-5">
      <div className="text-[2.5rem] font-semibold">
        <span className="text-nord-9">Ice</span>lang.
      </div>
      <div className="mt-2 px-4 text-paragraph">
        A minimal programming language inspired by lua and rust.
      </div>
      <div className="mt-8 flex">
        <a
          href="#quickstart"
          className="mr-4 px-4 py-3 flex items-center rounded-md font-semibold text-nord-1 bg-nord-5 hover:text-nord-5 hover:bg-nord-0 duration-200"
        >
          <FaCompass className="mr-1" />
          Quickstart
        </a>
        <a
          href="#playground"
          className="px-4 py-3 flex items-center rounded-md font-semibold text-nord-0 bg-nord-9 hover:text-nord-5 hover:bg-nord-0 duration-200"
        >
          <FaCode className="mr-1" />
          Playground
        </a>
      </div>
    </div>
  );
};

export default Landing;
