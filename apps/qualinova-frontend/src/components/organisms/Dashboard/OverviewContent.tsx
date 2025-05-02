import React from 'react';
import { LineChart, Line, XAxis, YAxis, CartesianGrid, Tooltip, ResponsiveContainer } from 'recharts';
import { Clock, Award, RefreshCcw, Search, FileText, FilePlus, FileCheck, FileX } from 'lucide-react';

type CertificateStatus = 'Verified' | 'Pending' | 'Expired';

interface Certificate {
  name: string;
  company: string;
  date: string;
  status: CertificateStatus;
}

interface MetricsData {
  total: {
    count: number;
    percentChange: string;
  };
  active: {
    count: number;
    percentText: string;
  };
  pending: {
    count: number;
    statusText: string;
  };
  expired: {
    count: number;
    percentText: string;
  };
}

interface ChartDataPoint {
  name: string;
  value: number;
}

interface DashboardData {
  metrics: MetricsData;
  chartData: ChartDataPoint[];
  recentCertificates: Certificate[];
}

// Sample data
const dummyData: DashboardData = {
  metrics: {
    total: {
      count: 142,
      percentChange: "+22% from last month"
    },
    active: {
      count: 124,
      percentText: "87% of total certificates"
    },
    pending: {
      count: 8,
      statusText: "Awaiting blockchain confirmation"
    },
    expired: {
      count: 10,
      percentText: "7% of total certificates"
    }
  },
  chartData: [
    { name: 'Jan', value: 12 },
    { name: 'Feb', value: 19 },
    { name: 'Mar', value: 23 },
    { name: 'Apr', value: 27 },
    { name: 'May', value: 36 },
    { name: 'Jun', value: 32 },
    { name: 'Jul', value: 38 },
    { name: 'Aug', value: 43 },
    { name: 'Sep', value: 55 },
  ],
  recentCertificates: [
    { name: "ISO 9001 Compliance", company: "Global Industries", date: "10/12/2023", status: "Verified" },
    { name: "Food Safety Certification", company: "Fresh Organics", date: "8/17/2023", status: "Verified" },
    { name: "Environmental Management", company: "Green Solutions", date: "7/05/2023", status: "Pending" },
    { name: "Quality Assurance", company: "Tech Standards", date: "3/21/2023", status: "Expired" },
    { name: "Health & Safety Standard", company: "Industrial Systems", date: "1/6/2023", status: "Verified" }
  ]
};

export default function OverviewContent() {
  // Badge component for status indicators
  const StatusBadge = ({ status }: { status: CertificateStatus }) => {
    const badgeStyles = {
      Verified: "bg-[#DCFCE7] text-[#166534] ",
      Pending: "bg-[#FFEDD5] text-[#9A3412]",
      Expired: "bg-[#7F1D1D] text-[#F8FAFC]"
    };

    return (
      <span className={`px-2 py-0.5 rounded-md text-xs font-medium ${badgeStyles[status]}`}>
        {status}
      </span>
    );
  };

  return (
    <div className="min-h-screen text-white p-2 sm:p-4 md:p-6">
      <div className="max-w-7xl mx-auto space-y-4 sm:space-y-6">
        {/* Metrics Section */}
        <div className="grid grid-cols-1 sm:grid-cols-2 lg:flex lg:flex-row w-full gap-3 sm:gap-4 lg:space-x-0">
          <div className="rounded-lg p-3 sm:p-4 border border-[#1E293B] flex-1">
            <div className="flex justify-between items-start">
              <div>
                <p className="text-[#F8FAFC] text-base sm:text-lg md:text-xl mb-1">Total Certificates</p>
                <h3 className="text-[#F8FAFC] text-xl sm:text-2xl font-semibold">{dummyData.metrics.total.count}</h3>
                <p className="text-[#94A3B8] text-xs">{dummyData.metrics.total.percentChange}</p>
              </div>
              <div className="text-[#2563EB]">
                <span className="flex items-center justify-center h-8 w-8 rounded-full bg-blue-500/10">
                  <Award size={18} />
                </span>
              </div>
            </div>
          </div>

          <div className="rounded-lg p-3 sm:p-4 border border-[#1E293B] flex-1 lg:ml-0">
            <div className="flex justify-between items-start">
              <div>
                <p className="text-[#F8FAFC] text-base sm:text-lg md:text-xl mb-1">Active Certificates</p>
                <h3 className="text-[#F8FAFC] text-xl sm:text-2xl font-semibold">{dummyData.metrics.active.count}</h3>
                <p className="text-[#94A3B8] text-xs">{dummyData.metrics.active.percentText}</p>
              </div>
              <div className="text-[#16A34A]">
                <span className="flex items-center justify-center h-8 w-8 rounded-full bg-green-500/10">
                  <FileCheck size={18} />
                </span>
              </div>
            </div>
          </div>

          <div className="rounded-lg p-3 sm:p-4 border border-[#1E293B] flex-1 lg:ml-0">
            <div className="flex justify-between items-start">
              <div>
                <p className="text-[#F8FAFC] text-base sm:text-lg md:text-xl mb-1">Pending Certificates</p>
                <h3 className="text-[#F8FAFC] text-xl sm:text-2xl font-semibold">{dummyData.metrics.pending.count}</h3>
                <p className="text-gray-400 text-xs">{dummyData.metrics.pending.statusText}</p>
              </div>
              <div className="text-[#EA580C]">
                <span className="flex items-center justify-center h-8 w-8 rounded-full bg-orange-500/10">
                  <Clock size={18} />
                </span>
              </div>
            </div>
          </div>

          <div className="rounded-lg p-3 sm:p-4 border border-[#1E293B] flex-1 lg:ml-0">
            <div className="flex justify-between items-start">
              <div>
                <p className="text-[#F8FAFC] text-base sm:text-lg md:text-xl mb-1">Expired Certificates</p>
                <h3 className="text-[#F8FAFC] text-xl sm:text-2xl font-semibold">{dummyData.metrics.expired.count}</h3>
                <p className="text-[#94A3B8] text-xs">{dummyData.metrics.expired.percentText}</p>
              </div>
              <div className="text-[#DC2626]">
                <span className="flex items-center justify-center h-8 w-8 rounded-full bg-red-500/10">
                  <FileX size={18} />
                </span>
              </div>
            </div>
          </div>
        </div>

        {/* Main Content Grid */}
        <div className="rounded-lg border border-[#1E293B] p-0 overflow-hidden">
          <div className="grid grid-cols-1 lg:grid-cols-12">
            {/* Certificate Activity Chart */}
            <div className="lg:col-span-7 p-3 sm:p-4 lg:border-r border-[#1E293B]">
              <h2 className="text-[#F8FAFC] text-base sm:text-lg font-medium mb-2 sm:mb-4">Certificate Activity</h2>
              <div className="h-48 sm:h-56 md:h-64">
                <ResponsiveContainer width="100%" height="100%">
                  <LineChart
                    data={dummyData.chartData}
                    margin={{ top: 10, right: 10, left: 0, bottom: 20 }}
                  >
                    <CartesianGrid
                      strokeDasharray="3 3"
                      vertical={false}
                      stroke="#333"
                    />
                    <XAxis
                      dataKey="name"
                      axisLine={false}
                      tickLine={false}
                      tick={{ fill: '#6B7280', fontSize: 12 }}
                      dy={10}
                    />
                    <YAxis
                      axisLine={false}
                      tickLine={false}
                      tick={{ fill: '#6B7280', fontSize: 12 }}
                      width={30}
                      domain={[0, 60]}
                      ticks={[0, 15, 30, 45, 60]}
                    />
                    <Tooltip
                      contentStyle={{
                        backgroundColor: '#111827',
                        borderColor: '#374151',
                        borderRadius: '4px',
                        padding: '8px'
                      }}
                      labelStyle={{
                        color: 'white',
                        fontWeight: '500'
                      }}
                      itemStyle={{
                        color: '#3B82F6'
                      }}
                      cursor={false}
                    />
                    <Line
                      type="monotone"
                      dataKey="value"
                      stroke="#3B82F6"
                      strokeWidth={2}
                      dot={false}
                      activeDot={{ r: 4, fill: '#3B82F6' }}
                      isAnimationActive={true}
                      animationDuration={1000}
                    />
                  </LineChart>
                </ResponsiveContainer>
              </div>
            </div>

            {/* Recent Certificates */}
            <div className="lg:col-span-5 p-3 sm:p-4">
              <div className="flex justify-between items-center mb-2 sm:mb-4">
                <h2 className="text-[#F8FAFC] text-base sm:text-lg font-medium">Recent Certificates</h2>
              </div>
              <p className="text-[#94A3B8] text-xs mb-2 sm:mb-3">Recently created or updated certificates</p>
              <div className="overflow-x-auto">
                {dummyData.recentCertificates.map((cert, index) => (
                  <div key={index} className="flex justify-between items-center py-2 sm:py-3 border-b border-gray-800">
                    <div className="flex flex-col">
                      <div className="flex items-center">
                        {cert.status === "Verified" && <FileCheck size={14} className="text-[#16A34A] mr-2" />}
                        {cert.status === "Pending" && <Clock size={14} className="text-[#EA580C] mr-2" />}
                        {cert.status === "Expired" && <FileX size={14} className="text-[#DC2626] mr-2" />}
                        <h4 className="text-[#F8FAFC] font-medium text-sm sm:text-base truncate max-w-[120px] sm:max-w-full">{cert.name}</h4>
                      </div>
                      <p className="text-[#94A3B8] text-xs">{cert.company}</p>
                    </div>
                    <div className="flex items-center gap-2 sm:gap-4">
                      <span className="text-[#94A3B8] pr-8 text-xs hidden sm:inline">{cert.date}</span>
                      <StatusBadge status={cert.status} />
                    </div>
                  </div>
                ))}
              </div>
              <div className="mt-3 sm:mt-4 flex border border-[#1E293B] p-2 sm:p-3 rounded justify-center">
                <button className="text-[#F8FAFC] text-xs sm:text-sm hover:text-blue-400 transition-colors font-medium">
                  View All Certificates
                </button>
              </div>
            </div>
          </div>
        </div>

        {/* Quick Actions */}
        <div className="mt-4 sm:mt-6 border border-[#1E293B] rounded-lg p-3 sm:p-4">
          <h2 className="text-white text-base sm:text-lg font-medium mb-3 sm:mb-4">Quick Actions</h2>
          <div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-3 sm:gap-4">
            <div className="bg-[#2563EB] rounded-lg p-3 sm:p-4 text-left flex items-center hover:bg-blue-700 transition-colors w-full cursor-pointer">
              <div className="rounded-full p-2 mr-2">
                <FilePlus size={16} className="text-[#0F172A]" />
              </div>
              <div>
                <h3 className="text-[#0F172A] font-medium text-sm sm:text-base">Create Certificate</h3>
                <p className="text-[#BFDBFE] text-xs">Generate a new certificate</p>
              </div>
            </div>

            <div className="border border-[#1E293B] rounded-lg p-3 sm:p-4 text-left flex items-center hover:bg-gray-800 transition-colors w-full cursor-pointer">
              <div className="rounded-full p-2 mr-2">
                <Search size={16} className="text-white" />
              </div>
              <div>
                <h3 className="text-white font-medium text-sm sm:text-base">Verify Certificate</h3>
                <p className="text-[#94A3B8] text-xs">Check certificate validity</p>
              </div>
            </div>

            <div className="border border-[#1E293B] rounded-lg p-3 sm:p-4 text-left flex items-center hover:bg-gray-800 transition-colors w-full cursor-pointer sm:col-span-2 md:col-span-1">
              <div className="rounded-full p-2 mr-2">
                <RefreshCcw size={16} className="text-white" />
              </div>
              <div>
                <h3 className="text-white font-medium text-sm sm:text-base">Renew Certificate</h3>
                <p className="text-[#94A3B8] text-xs">Extend certificate validity</p>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}