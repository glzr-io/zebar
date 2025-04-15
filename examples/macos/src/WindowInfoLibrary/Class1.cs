using System;
using System.Collections.Generic;
using System.Linq;
using System.Runtime.InteropServices;
using System.Text;
using System.Windows.Automation;

namespace WindowInfoLibrary
{
    public static class WindowReader
    {
        // Import necessary Windows functions
        [DllImport("user32.dll")]
        private static extern IntPtr GetForegroundWindow();

        [DllImport("user32.dll", SetLastError = true, CharSet = CharSet.Auto)]
        static extern int GetWindowText(IntPtr hWnd, StringBuilder lpString, int nMaxCount);

        [DllImport("user32.dll", SetLastError = true, CharSet = CharSet.Auto)]
        static extern int GetWindowTextLength(IntPtr hWnd);

        // Unmanaged export to get active window title
        [UnmanagedCallersOnly(EntryPoint = "GetActiveWindowTitle")]
        public static IntPtr GetActiveWindowTitle_Unmanaged()
        {
            IntPtr activeWindowHandle = GetForegroundWindow();
            if (activeWindowHandle == IntPtr.Zero)
            {
                return IntPtr.Zero;
            }

            int length = GetWindowTextLength(activeWindowHandle);
            StringBuilder sb = new StringBuilder(length + 1);
            if (GetWindowText(activeWindowHandle, sb, sb.Capacity) > 0)
            {
                IntPtr ptr = Marshal.StringToHGlobalUni(sb.ToString());
                return ptr;
            }

            return IntPtr.Zero;
        }

        // Unmanaged export to free memory allocated for window title
        [UnmanagedCallersOnly(EntryPoint = "FreeActiveWindowTitle")]
        public static void FreeActiveWindowTitle_Unmanaged(IntPtr ptr)
        {
            if (ptr != IntPtr.Zero)
            {
                Marshal.FreeHGlobal(ptr);
            }
        }

        // Export for getting the active window top-level menu items
        [UnmanagedCallersOnly(EntryPoint = "GetActiveWindowTopLevelMenuItems")]
        public static IntPtr GetActiveWindowTopLevelMenuItems_Unmanaged()
        {
            var menuItems = GetActiveWindowTopLevelMenuItemsInternal();

            if (menuItems != null)
            {
                // Allocate unmanaged memory for the menu items' strings
                int totalLength = 0;
                foreach (var item in menuItems)
                {
                    totalLength += item.Length + 1; // +1 for null terminator
                }

                IntPtr unmanagedArray = Marshal.AllocHGlobal(totalLength);
                try
                {
                    IntPtr current = unmanagedArray;
                    foreach (var item in menuItems)
                    {
                        // Copy the string to unmanaged memory
                        Marshal.Copy(System.Text.Encoding.UTF8.GetBytes(item + "\0"), 0, current, item.Length + 1);
                        current = IntPtr.Add(current, item.Length + 1);
                    }
                }
                catch
                {
                    Marshal.FreeHGlobal(unmanagedArray); // Clean up on failure
                    return IntPtr.Zero; // Return zero on failure
                }

                return unmanagedArray; // Return the pointer to the strings                
            }

            return IntPtr.Zero;
        }

        // Export function to free memory allocated for menu items
        [UnmanagedCallersOnly(EntryPoint = "FreeActiveWindowMenuItems")]
        public static void FreeActiveWindowMenuItems_Unmanaged(IntPtr arrayPtr, int count)
        {
            if (arrayPtr != IntPtr.Zero && count > 0)
            {
                IntPtr currentPtr;
                for (int i = 0; i < count; i++)
                {
                    currentPtr = Marshal.ReadIntPtr(arrayPtr, i * Marshal.SizeOf(typeof(IntPtr)));
                    Marshal.FreeHGlobal(currentPtr);
                }
                Marshal.FreeHGlobal(arrayPtr);
            }
        }

        // Internal method to get the menu items of the active window
        public static List<string>? GetActiveWindowTopLevelMenuItemsInternal()
        {
            IntPtr activeWindowHandle = GetForegroundWindow();
            if (activeWindowHandle == IntPtr.Zero)
            {
                return null;
            }

            try
            {
                AutomationElement? appElement = AutomationElement.FromHandle(activeWindowHandle);
                if (appElement != null)
                {
                    Condition menuBarCondition = new PropertyCondition(AutomationElement.ControlTypeProperty, ControlType.MenuBar);
                    AutomationElement? menuContainer = FindElementRecursive(appElement, menuBarCondition);

                    if (menuContainer != null)
                    {
                        Condition menuItemCondition = new PropertyCondition(AutomationElement.ControlTypeProperty, ControlType.MenuItem);
                        AutomationElementCollection menuItems = menuContainer.FindAll(TreeScope.Children, menuItemCondition);
                        return menuItems.Cast<AutomationElement>()
                                        .Select(item => item.Current.Name)
                                        .Where(name => !string.IsNullOrEmpty(name))
                                        .ToList();
                    }
                    else
                    {
                        return new List<string>();
                    }
                }
            }
            catch (Exception ex)
            {
                return null;
            }

            return new List<string>();
        }

        // Helper function to recursively search for an element within a parent
        public static AutomationElement? FindElementRecursive(AutomationElement parentElement, Condition condition)
        {
            if (parentElement == null) return null;

            try
            {
                AutomationElement? element = parentElement.FindFirst(TreeScope.Descendants, condition);
                if (element != null) return element;
            }
            catch (Exception) { }

            try
            {
                AutomationElement? element = parentElement.FindFirst(TreeScope.Children, condition);
                if (element != null) return element;
            }
            catch (Exception) { }

            return null; // Not found
        }
    }
}
