<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>GET Users By Id</name>
   <tag></tag>
   <elementGuidId>27c730d2-d0ed-4f7b-b296-e73edf7e1e6b</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <katalonVersion>8.3.5</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>https://jsonplaceholder.typicode.com/users/3</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()


WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)


WS.verifyElementPropertyValue(response, 'id', '3')
WS.verifyElementPropertyValue(response, 'name', 'Clementine Bauch')
WS.verifyElementPropertyValue(response, 'username', 'Samantha')
WS.verifyElementPropertyValue(response, 'email', 'Nathan@yesenia.net')
WS.verifyElementPropertyValue(response, 'address.street', 'Douglas Extension')
WS.verifyElementPropertyValue(response, 'address.suite', 'Suite 847')
WS.verifyElementPropertyValue(response, 'address.city', 'McKenziehaven')
WS.verifyElementPropertyValue(response, 'address.zipcode', '59590-4157')
WS.verifyElementPropertyValue(response, 'address.geo.lat', '-68.6102')
WS.verifyElementPropertyValue(response, 'address.geo.lng', '-47.0653')
WS.verifyElementPropertyValue(response, 'phone', '1-463-123-4447')
WS.verifyElementPropertyValue(response, 'website', 'ramiro.info')
WS.verifyElementPropertyValue(response, 'company.name', 'Romaguera-Jacobson')
WS.verifyElementPropertyValue(response, 'company.catchPhrase', 'Face to face bifurcated interface')
WS.verifyElementPropertyValue(response, 'company.bs', 'e-enable strategic applications')</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
