type props = {
  children: React.ReactNode;
};

const Container: React.FC<props> = ({ children }) => {
  return <div className="min-h-screen flex flex-col bg-nord-1">{children}</div>;
};

export default Container;
