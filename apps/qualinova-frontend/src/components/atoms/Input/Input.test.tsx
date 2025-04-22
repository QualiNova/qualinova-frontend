import React from "react";
import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import Input from "./Input";
import "@testing-library/jest-dom";

const mockRegister = jest.fn().mockImplementation((name) => ({
  name,
  onChange: jest.fn(),
  onBlur: jest.fn(),
  ref: jest.fn(),
}));

describe("Input Component", () => {
  it("renders correctly with label", () => {
    render(<Input label="Email" />);

    expect(screen.getByText("Email")).toBeInTheDocument();
    expect(screen.getByRole("textbox")).toBeInTheDocument();
  });

  it("applies correct attributes from props", () => {
    render(
      <Input
        label="Email"
        placeholder="name@example.com"
        type="email"
        data-testid="email-input"
      />
    );

    const input = screen.getByTestId("email-input");
    expect(input).toHaveAttribute("placeholder", "name@example.com");
    expect(input).toHaveAttribute("type", "email");
  });

  it("displays error message when provided", () => {
    const errorMessage = "Email is required";
    render(<Input label="Email" error={errorMessage} />);

    expect(screen.getByText(errorMessage)).toBeInTheDocument();
    expect(screen.getByText(errorMessage)).toHaveClass("text-red-500");
  });

  it("works with react-hook-form register", () => {
    const { rerender } = render(
      <Input
        label="Email"
        placeholder="name@example.com"
        {...mockRegister("email")}
      />
    );

    expect(mockRegister).toHaveBeenCalledWith("email");

    rerender(
      <Input
        label="Email"
        placeholder="name@example.com"
        {...mockRegister("email")}
        error="Email is invalid"
      />
    );

    expect(screen.getByText("Email is invalid")).toBeInTheDocument();
  });

  it("allows typing in the input field", async () => {
    const user = userEvent.setup();
    render(<Input label="Email" data-testid="email-input" />);

    const input = screen.getByTestId("email-input");
    await user.type(input, "test@example.com");

    expect(input).toHaveValue("test@example.com");
  });

  it("forwards the ref correctly", () => {
    const ref = React.createRef<HTMLInputElement>();
    render(<Input label="Email" ref={ref} />);

    expect(ref.current).not.toBeNull();
    expect(ref.current).toBeInstanceOf(HTMLInputElement);
  });
});
