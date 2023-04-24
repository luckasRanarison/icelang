import init from "../pkg";
import { useEffect } from "react";
import { Container, Navbar, Playground } from "./components";

const App = () => {
  useEffect(() => {
    init();
  }, []);

  return (
    <Container>
      <Navbar />
      <Playground />
    </Container>
  );
};

export default App;
