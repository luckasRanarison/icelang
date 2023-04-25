import { FaGithub, FaBug } from "react-icons/fa";

const Footer = () => {
  return (
    <div className="mt-8 p-12 w-full flex flex-col items-center text-center text-nord-5 leading-8">
      <div className="flex">
        Found a{" "}
        <span className="flex items-center font-bold text-nord-12">
          <FaBug className="mx-1" /> bug
        </span>
        ? Report it
        <a
          href="http://github.com/luckasranarison/icelang/issues"
          className="font-bold ml-1 underline decoration-2"
        >
          here
        </a>
      </div>
      <div>Copyright© 2023 Icelang. Licensed under the MIT License.</div>
    </div>
  );
};

export default Footer;
