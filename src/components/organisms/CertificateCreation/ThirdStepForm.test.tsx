import { render, screen } from "@testing-library/react";
import "@testing-library/jest-dom";
import userEvent from "@testing-library/user-event";
import ThirdStepForm from "./ThirdStepForm";

describe("ThirdStepForm Component", () => {
    test("renders without crashing", () => {
        render(<ThirdStepForm />);
        expect(screen.getByText("Certificate Details")).toBeInTheDocument();
        expect(screen.getByText("Step 3 of 3")).toBeInTheDocument();
    });

    test("displays certificate details", () => {
        render(<ThirdStepForm />);
        expect(screen.getByText("ISO 9001 Quality Management")).toBeInTheDocument();
        expect(screen.getByText("Quality Management")).toBeInTheDocument();
        expect(screen.getByText("Acme Corporation")).toBeInTheDocument();
    });

    test("buttons are clickable", async () => {
        render(<ThirdStepForm />);
        const prevButton = screen.getByText("Previous Step");
        const createButton = screen.getByText("Create Certificate");

        await userEvent.click(prevButton);
        await userEvent.click(createButton);

        expect(prevButton).toBeEnabled();
        expect(createButton).toBeEnabled();
    });
});
