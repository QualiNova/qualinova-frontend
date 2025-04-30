

import React from 'react'

const page = () => {
  return (
      <div className='flex *:mt-14 flex-col items-center justify-center'>
          <div style={{width: "45%"}} className='flex  space-y-4 flex-col items-center'>
              <div className='text-4xl font-bold'>Certificate Verification</div>
              <div className='text-gray-400 text-center text-xl'>
                  Verify the authenticity of any QualiNova certificate using its unique ID or by scanning the
QR code
              </div>
          </div>
          {/*  */}
          <div>
              <div>
                  <button type="button">Verify by ID</button>
                  <button type="button">Verify by QR Code</button>

              </div>
          </div>
    </div>
  )
}

export default page