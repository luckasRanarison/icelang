import CodeBlock from "./CodeBlock";

type props = {
  header: string;
  paragraph: string;
  code: string;
};

const Section: React.FC<props> = ({ header, paragraph, code }) => {
  return (
    <div>
      <div className="mt-6 mb-2 w-full text-title text-nord-6 font-semibold">
        {header}
      </div>
      <p className="mb-4 scrollYtext-paragraph text-nord-4">{paragraph}</p>
      <CodeBlock value={code} />
    </div>
  );
};

export default Section;
