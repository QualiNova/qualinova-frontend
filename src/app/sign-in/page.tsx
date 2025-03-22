import AuthForm from '../components/AuthForm'

const SignInPage = () => {
    return (
        <div className='flex items-center justify-center h-screen w-full bg-black text-white'>
            <div className="text-white flex flex-col max-w-5xl">
                <h2 className="font-semibold text-3xl text-center">Welcome Back</h2>
                <p className='text-lg text-center text-gray-400'>Enter your credentials to sign in to your account</p>

                <AuthForm
                    type="SIGN_IN"
                    defaultValues={{ email: '', password: '' }}
                />
            </div>
        </div>
    )
}

export default SignInPage