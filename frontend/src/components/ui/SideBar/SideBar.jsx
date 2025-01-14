import React, { useEffect, useRef, useState } from "react";
import { Layout, Menu, Avatar, Typography } from "antd"; // Importing Ant Design components
import { LogoutOutlined } from '@ant-design/icons'; // Importing Ant Design icons
import "./SideBar.css"; // Import your custom CSS
import Sider from "antd/es/layout/Sider";
import Icon from "../Icon/Icon";
import getLetterColor from '../../../utils/colors';
import Button from "../Button/Button";
const { Text } = Typography;


const Sidebar = ({ screens, currentScreen, onEnterScreen, ...props }) => {
    
    const { color, backgroundColor } = getLetterColor(props.user.firstname[0]||props.user.name[0]||'O');
    const [sidebarOpen, setSidebarOpen] = useState(false);
    const [currentmenuitem, setCurrentMenuItem] = useState(currentScreen?.text || screens[0].text);
    // Create a reference to the sidebar element
    const sidebarRef = useRef(null);
    const clickMenuItem =(key) =>{
        onEnterScreen(screens.find(screen => screen.text === key));
        setCurrentMenuItem(key);
    }
    // Close sidebar if clicked outside
    useEffect(() => {
        const handleClickOutside = (event) => {
          if (sidebarRef.current && !sidebarRef.current.contains(event.target)) {
              setSidebarOpen(false); // Close the sidebar when clicked outside
          }
        };

    // Adding the event listener
    document.addEventListener("mousedown", handleClickOutside);

    // Cleanup event listener on component unmount
    return () => {
      document.removeEventListener("mousedown", handleClickOutside);
    };
  }, []);

  return (
    <Layout style={{ height: '100%' }}>
      <Sider
        width={sidebarOpen ? 250 : 78} // Conditional width based on sidebarOpen
        className={`sidebar ${sidebarOpen ? 'sidebar-open' : ''}`}
        // collapsible
        collapsed={!sidebarOpen}
        onCollapse={() => setSidebarOpen(!sidebarOpen)}
        ref={sidebarRef}  // Attach ref to Sider
        theme="light"
      >
        
        {/* Sidebar Header */}
        <div className="sidebar-header">
          <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
            <Avatar 
              src="" 
              alt="Profile Picture" 
              shape="circle" 
              size={50} 
              style={{ marginRight: 10 }}
            />
            <Text className={`sidebar-header-name ${!sidebarOpen?'sidebar-header-name-hidden':''}`} strong
            >Screens</Text>
          </div>
        </div>

        {/* screens List */}
        <Menu
          // mode="inline"
          selectedKeys={[currentScreen.text]}
          onClick={({ key }) => clickMenuItem(key)}
          style={{ paddingTop: 20 }}
          className="screen-list"
        >
          {screens.map((screen, index) => (
            <Menu.Item 
              
              key={screen.text} 
              icon={<Icon name={screen.icon}/>}
              className={`screen-item ${currentmenuitem === screen.text ? 'screen-item-active' : ''}`}
            >
              <Text className={`screen-item-name ${!sidebarOpen?'screen-item-name-hidden':''}`} strong
                >{screen.text}</Text>
                
            </Menu.Item>
          ))}
        </Menu>
        
        
        
        <div className="sidebar-bottom" style={{ display: 'flex', justifyContent: 'space-between', flexDirection:'column'}}>
          <div className="sidebar-close-menu">
              <div style={{ display: 'flex', justifyContent: 'space-around', alignItems: 'center', width:'100%'}}>
                  <Button icon={!sidebarOpen?'fa-bars':'fa-xmark'} border={'none'} onClick={() => setSidebarOpen(!sidebarOpen)}>
                    <Text className={`sidebar-close-menu-name ${!sidebarOpen?'sidebar-close-menu-name-hidden':''}`} strong
                      >{!sidebarOpen?'show':'hide'}</Text>
                    </Button>
              </div>
          </div>
          {/* Sidebar Footer */}
          <div className="sidebar-footer">
            {!props?.user.isloggedin ? (
              <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center'}}>
                <Avatar 
                    src="o" 
                    size={50} 
                    alt="oscar oguledo" shape="circle" 
                    className="sidebar-footer-img"
                    style={{ color: color, backgroundColor: backgroundColor}} children={'O'} 
                />
                <Text className={`sidebar-footer-name ${!sidebarOpen?'sidebar-footer-name-hidden':''}`} strong
                  >oscar oguledo 
                    <Icon name='fa-ellipsis-vertical' style={{marginLeft:'50px'}}/>
                  </Text>
                  
                
              </div>
            ) : (
              <Button 
                type="primary"
                icon={<LogoutOutlined />} 
                style={{ width: '100%' }}
                onClick={() => {/* Handle logout */}}
              >
                  <Text className={`sidebar-footer-name ${!sidebarOpen?'sidebar-footer-name-hidden':'' }`}>Logout</Text>{/* Show "Logout" when sidebar is open */}
              </Button>
            )}
          </div>
        </div>
      </Sider>
    </Layout>
  );
};

export default Sidebar;
