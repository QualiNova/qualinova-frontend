import { render, screen, fireEvent } from "@testing-library/react";
import PasswordInput from "../components/ui/PasswordInput";
import "@testing-library/jest-dom";

describe("PasswordInput Component", () => {
  test("renders the password input field", () => {
    render(<PasswordInput label="Password" />);
    expect(screen.getByLabelText(/password/i)).toBeInTheDocument();
  });

  test("toggles password visibility when clicking the eye icon", () => {
    render(<PasswordInput label="Password" />);

    const passwordField = screen.getByLabelText(/password/i);
    const toggleButton = screen.getByRole("button");

    expect(passwordField).toHaveAttribute("type", "password");

    fireEvent.click(toggleButton);
    expect(passwordField).toHaveAttribute("type", "text");

    fireEvent.click(toggleButton);
    expect(passwordField).toHaveAttribute("type", "password");
  });

  test("displays an error message when provided", () => {
    render(<PasswordInput label="Password" error="This field is required" />);
    expect(screen.getByText(/this field is required/i)).toBeInTheDocument();
  });
});
