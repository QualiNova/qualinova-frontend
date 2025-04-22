import React from 'react';
import { render, screen } from '@testing-library/react';
import FeatureCard from './FeatureCard';

describe('FeatureCard Component', () => {
  const mockIcon = <svg data-testid="mock-icon" />;

  const defaultProps = {
    title: 'Test Title',
    description: 'Test Description',
    icon: mockIcon,
    iconBgColor: 'bg-blue-500',
    iconTextColor: 'text-white',
  };

  it('renders the component with all props correctly', () => {
    render(<FeatureCard {...defaultProps} />);

    // Check if the title is rendered
    expect(screen.getByText('Test Title')).toBeInTheDocument();

    // Check if the description is rendered
    expect(screen.getByText('Test Description')).toBeInTheDocument();

    // Check if the icon is rendered
    expect(screen.getByTestId('mock-icon')).toBeInTheDocument();

    // Check if the icon container has the correct background color
    const iconContainer = screen.getByTestId('mock-icon').parentElement;
    expect(iconContainer).toHaveClass('bg-blue-500');

    // Check if the icon has the correct text color
    expect(screen.getByTestId('mock-icon')).toHaveClass('text-white');
  });

  it('applies the correct styles to the icon container and icon', () => {
    const customProps = {
      ...defaultProps,
      iconBgColor: 'bg-red-500',
      iconTextColor: 'text-black',
    };

    render(<FeatureCard {...customProps} />);

    // Check if the icon container has the updated background color
    const iconContainer = screen.getByTestId('mock-icon').parentElement;
    expect(iconContainer).toHaveClass('bg-red-500');

    // Check if the icon has the updated text color
    expect(screen.getByTestId('mock-icon')).toHaveClass('text-black');
  });

  it('renders the component with the correct structure', () => {
    render(<FeatureCard {...defaultProps} />);

    // Check if the component has the correct structure
    const featureCard = screen.getByRole('article');
    expect(featureCard).toHaveClass('bg-gray-800/50');
    expect(featureCard).toHaveClass('rounded-lg');
    expect(featureCard).toHaveClass('p-8');
    expect(featureCard).toHaveClass('border');
    expect(featureCard).toHaveClass('border-gray-700/50');
    expect(featureCard).toHaveClass('flex');
    expect(featureCard).toHaveClass('flex-col');
    expect(featureCard).toHaveClass('items-center');
    expect(featureCard).toHaveClass('text-center');
  });
});