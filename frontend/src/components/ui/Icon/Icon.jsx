import React from 'react';
import PropTypes from 'prop-types';  // Import PropTypes to define expected props
import './Icon.css';  // Import any CSS for icon styling

const Icon = ({
  name, 
  onClick,
  className,
  size,
  style,
  props
}) => {
  const Size = `icon-${size}`
  return (
    <i
      className={`icon fa-light ${name} ${className} ${Size}`}
      onClick={onClick}
      style={style?style:{}}
      {...props}
    />
  );
};

// Define prop types to ensure that the props are used correctly
Icon.propTypes = {
    name: PropTypes.string.isRequired, // The icon name is required (FontAwesome class name)
    onClick: PropTypes.func, // onClick function (optional)
};

// Default props if no value is provided
Icon.default = {
  onClick: null,  // Default to no onClick handler
};

export default Icon;
