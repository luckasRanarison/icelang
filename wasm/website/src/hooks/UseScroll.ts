import { useState, useEffect } from "react";

export default function useScroll() {
  const [data, setData] = useState({
    scrollX: 0,
    scrollY: 0,
    lastX: 0,
    lastY: 0,
  });

  useEffect(() => {
    const handleScroll = () => {
      setData((last) => {
        return {
          scrollX: window.scrollX,
          scrollY: window.scrollY,
          lastX: last.scrollX,
          lastY: last.scrollY,
        };
      });
    };

    handleScroll();
    window.addEventListener("scroll", handleScroll);

    return () => {
      window.removeEventListener("scroll", handleScroll);
    };
  }, []);

  return data;
}
