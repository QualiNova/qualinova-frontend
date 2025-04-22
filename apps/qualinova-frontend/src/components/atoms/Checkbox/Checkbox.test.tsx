import { render, screen, fireEvent } from "@testing-library/react";
import Checkbox from "./Checkbox";
import "@testing-library/jest-dom";
import React from "react";

describe("Checkbox Component", () => {
  test("renders correctly with label", () => {
    render(<Checkbox label="Accept Terms" />);

    expect(screen.getByText("Accept Terms")).toBeInTheDocument();
    expect(screen.getByRole("checkbox")).toBeInTheDocument();
  });

  test("can be checked and unchecked", () => {
    render(<Checkbox label="Subscribe" />);
    const checkbox = screen.getByRole("checkbox");

    expect(checkbox).not.toBeChecked();

    fireEvent.click(checkbox);
    expect(checkbox).toBeChecked();

    fireEvent.click(checkbox);
    expect(checkbox).not.toBeChecked();
  });

  test("displays error message when provided", () => {
    render(<Checkbox label="I agree" error="This field is required" />);

    expect(screen.getByText("This field is required")).toBeInTheDocument();
    expect(screen.getByText("This field is required")).toHaveClass(
      "text-red-500"
    );
  });

  test("supports ref forwarding", () => {
    const ref = React.createRef<HTMLInputElement>();
    render(<Checkbox ref={ref} label="Test Ref" />);

    expect(ref.current).not.toBeNull();
    expect(ref.current?.tagName).toBe("INPUT");
  });
});
