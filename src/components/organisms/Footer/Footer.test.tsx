import { render, screen } from "@testing-library/react";
import Footer from "./Footer";
import "@testing-library/jest-dom";

describe("Footer Component", () => {
  it("renders the footer correctly", () => {
    render(<Footer />);

    expect(screen.getByText("QualiNova")).toBeInTheDocument();

    expect(screen.getByText("Terms")).toBeInTheDocument();
    expect(screen.getByText("Privacy")).toBeInTheDocument();
    expect(screen.getByText("Contact")).toBeInTheDocument();

    expect(
      screen.getByText(/© 2025 QualiNova. All rights reserved./i)
    ).toBeInTheDocument();

    const shieldIcon = screen.getByTestId("shield-icon");
    expect(shieldIcon).toBeInTheDocument();
  });
});
