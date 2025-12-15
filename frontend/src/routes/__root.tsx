import { Outlet, createRootRoute } from '@tanstack/react-router'

import Header from '../components/Header'
import { ClientContextProvider } from '@/contexts/ClientContext'
import Footer from '@/components/Footer'

export const Route = createRootRoute({
  component: () => (
    <>
      <ClientContextProvider>
        <div className="flex flex-col justify-between min-h-screen p-2">
          <div>
            <Header />
            <Outlet />
          </div>
          <Footer />
        </div>
      </ClientContextProvider>
    </>
  ),
})
