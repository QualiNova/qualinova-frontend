import React, { ReactNode } from 'react';

interface FeatureCardProps {
  title: string;
  description: string;
  icon: ReactNode;
  iconBgColor: string;
  iconTextColor: string;
}

const FeatureCard: React.FC<FeatureCardProps> = ({
  title,
  description,
  icon,
  iconBgColor,
  iconTextColor,
}) => {
  return (
    <div className="bg-gray-800/50 rounded-lg p-8 border border-gray-700/50 flex flex-col items-center text-center">
      <div className={`${iconBgColor} w-12 h-12 rounded-full flex items-center justify-center mb-6`}>
        <div className={`${iconTextColor}`}>
          {icon}
        </div>
      </div>
      <h3 className="text-xl font-semibold text-white mb-3">{title}</h3>
      <p className="text-gray-400 text-sm">{description}</p>
    </div>
  );
};

export default FeatureCard;