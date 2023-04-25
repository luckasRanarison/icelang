import init from "../pkg";
import { useEffect } from "react";
import {
  Container,
  Footer,
  Landing,
  Navbar,
  Overview,
  Playground,
} from "./components";

const App = () => {
  useEffect(() => {
    init();
  }, []);

  return (
    <Container>
      <Navbar />
      <Landing />
      <Overview />
      <Playground />
      <Footer />
    </Container>
  );
};

export default App;
