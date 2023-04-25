import { section } from "../static/overview";
import Section from "./Section";

const Overview = () => {
  return (
    <div id="quickstart" className="mt-10 flex flex-col items-center">
      <div className="px-4 w-full max-w-[700px]">
        {section.map((value, index) => (
          <Section
            key={index}
            header={value.header}
            paragraph={value.paragraph}
            code={value.code}
          />
        ))}
      </div>
    </div>
  );
};

export default Overview;
