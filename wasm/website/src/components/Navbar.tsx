import useScroll from "../hooks/UseScroll";
import { FaGithub } from "react-icons/fa";

const Navbar = () => {
  const { scrollY } = useScroll();

  return (
    <div
      className={`z-10 mb-2 px-8 py-4 top-0 sticky flex justify-between items-center text-nord-6 bg-nord-1 ${
        scrollY > 10 ? "shadow-md" : ""
      }`}
    >
      <div className="flex items-center font-semibold text-[1.5rem]">
        <img src="/mountain.png" alt="logo" className="w-6 mr-3" />
        <span className="text-nord-9">Ice</span>lang
      </div>
      <div>
        <a href="https://github.com/luckasranarison/icelang">
          <FaGithub size={25} className="hover:scale-125 duration-300" />
        </a>
      </div>
    </div>
  );
};

export default Navbar;
