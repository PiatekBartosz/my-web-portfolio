use leptos::*;

#[component]
pub fn TopNav() -> impl IntoView {
    view! {
   //      <nav class="bg-gray-900 dark:border-gray-700">
   //      <div class="max-w-screen-xl flex flex-wrap items-center justify-between mx-auto p-4">
   //         <a href="/" class="flex items-center space-x-3 rtl:space-x-reverse">
   //          <span class="self-center text-2xl font-semibold whitespace-nowrap dark:text-white">MyPortfolio</span>
   //         </a>
   //         <div class="hidden w-full md:block md:w-auto" id="navbar-dropdown">
   //            <ul class="flex flex-col font-medium p-4 md:p-0 mt-4 border border-gray-100 rounded-lg bg-gray-50 md:space-x-8 rtl:space-x-reverse md:flex-row md:mt-0 md:border-0 md:bg-white dark:bg-gray-800 md:dark:bg-gray-900 dark:border-gray-700">
   //               <li>
   //                  <a href="/" class="block py-2 px-3 text-white bg-blue-700 rounded md:bg-transparent md:text-blue-700 md:p-0 md:dark:text-blue-500 dark:bg-blue-600 md:dark:bg-transparent" aria-current="page">Home</a>
   //               </li>
   //               <li>
   //                  <a href="/about" class="block py-2 px-3 text-gray-900 rounded hover:bg-gray-100 md:hover:bg-transparent md:border-0 md:hover:text-blue-700 md:p-0 dark:text-white md:dark:hover:text-blue-500 dark:hover:bg-gray-700 dark:hover:text-white md:dark:hover:bg-transparent">About</a>
   //               </li>
   //               <li>
   //                   <button id="dropdownNavbarLink" data-dropdown-toggle="dropdownNavbar" class="flex items-center justify-between w-full py-2 px-3 text-gray-900 rounded hover:bg-gray-100 md:hover:bg-transparent md:border-0 md:hover:text-blue-700 md:p-0 md:w-auto dark:text-white md:dark:hover:text-blue-500 dark:focus:text-white dark:border-gray-700 dark:hover:bg-gray-700 md:dark:hover:bg-transparent">Dropdown <svg class="w-2.5 h-2.5 ms-2.5" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 10 6">
   //                         <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="m1 1 4 4 4-4"/>
   //                         </svg>
   //                   </button>
   //                   <div id="dropdownNavbar" class="z-10 hidden font-normal bg-white divide-y divide-gray-100 rounded-lg shadow w-44 dark:bg-gray-700 dark:divide-gray-600">
   //                         <ul class="py-2 text-sm text-gray-700 dark:text-gray-400" aria-labelledby="dropdownLargeButton">
   //                         <li>
   //                            <a href="#" class="block px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 dark:hover:text-white">Dashboard</a>
   //                         </li>
   //                         <li>
   //                            <a href="#" class="block px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 dark:hover:text-white">Settings</a>
   //                         </li>
   //                         <li>
   //                            <a href="#" class="block px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 dark:hover:text-white">Earnings</a>
   //                         </li>
   //                         </ul>
   //                         <div class="py-1">
   //                         <a href="#" class="block px-4 py-2 text-sm text-gray-700 hover:bg-gray-100 dark:hover:bg-gray-600 dark:text-gray-200 dark:hover:text-white">Sign out</a>
   //                         </div>
   //                   </div>
   //                </li>
   //               <li>
   //                  <a href="#" class="block py-2 px-3 text-gray-900 rounded hover:bg-gray-100 md:hover:bg-transparent md:border-0 md:hover:text-blue-700 md:p-0 dark:text-white md:dark:hover:text-blue-500 dark:hover:bg-gray-700 dark:hover:text-white md:dark:hover:bg-transparent">Pricing</a>
   //               </li>
   //               <li>
   //                  <a href="/jfdskal;" class="block py-2 px-3 text-gray-900 rounded hover:bg-gray-100 md:hover:bg-transparent md:border-0 md:hover:text-blue-700 md:p-0 dark:text-white md:dark:hover:text-blue-500 dark:hover:bg-gray-700 dark:hover:text-white md:dark:hover:bg-transparent">Contact</a>
   //               </li>
   //            </ul>
   //         </div>
   //      </div>
   //   </nav>
      <header class="text-gray-400 bg-gray-800 body-font">
         <div class="container mx-auto flex flex-wrap p-5 flex-col md:flex-row items-center">
            <a class="flex title-font font-medium items-center text-white mb-4 md:mb-0">
               <span class="ml-3 text-xl cursor-pointer">"MyWebPortfolio"</span>
            </a>
            <nav class="md:ml-auto flex flex-wrap items-center cursor-pointer text-base justify-right">
                  <a class="mr-5 hover:text-white" href="/">"Home"</a>
                  <a class="mr-5 hover:text-white" href="about">"About"</a>
                  <a class="mr-5 hover:text-white" href="projects">"Projects"</a>
                  <a class="mr-5 hover:text-white" href="dsa">"Data Structures & Algorithms"</a>
                  <a class="mr-5 hover:text-white" href="hire">"Hire me"</a>
            </nav>
         </div>
      </header>
      
    }
}