const { generateDefaultResponseObject } = require("../utils/defaultResponseObject").default

exports.get_api_status = (req, res) => {
    return res.status(200).json(generateDefaultResponseObject({ 
        success: true, 
        message: 'API is up and running',
    }))
}