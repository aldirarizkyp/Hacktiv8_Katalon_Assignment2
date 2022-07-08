<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>POST Add Users</name>
   <tag></tag>
   <elementGuidId>75e73790-6fed-48e9-8207-d7bf691084e7</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;name\&quot;: \&quot;Aldiransyah Rizky\&quot;,\n    \&quot;username\&quot;: \&quot;aldirarizkyp\&quot;,\n    \&quot;email\&quot;: \&quot;aldira@gmail.com\&quot;,\n    \&quot;address\&quot;: {\n      \&quot;street\&quot;: \&quot;Patal Senayan\&quot;,\n      \&quot;suite\&quot;: \&quot;Home\&quot;,\n      \&quot;city\&quot;: \&quot;Jakarta\&quot;,\n      \&quot;zipcode\&quot;: \&quot;12110\&quot;,\n      \&quot;geo\&quot;: {\n        \&quot;lat\&quot;: \&quot;-37\u003d0.321\&quot;,\n        \&quot;lng\&quot;: \&quot;70.2346\&quot;\n      }\n    },\n    \&quot;phone\&quot;: \&quot;086767675867\&quot;,\n    \&quot;website\&quot;: \&quot;aldraaa.id\&quot;,\n    \&quot;company\&quot;: {\n      \&quot;name\&quot;: \&quot;PT Aldira\&quot;,\n      \&quot;catchPhrase\&quot;: \&quot;Multi-layered client-server neural-net\&quot;,\n      \&quot;bs\&quot;: \&quot;harness real-time e-markets\&quot;\n    }\n  }&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>f06ee1a6-f197-451b-b274-bd056cd74032</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.3.5</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://jsonplaceholder.typicode.com/users</restUrl>
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

assertThat(response.getStatusCode()).isIn(Arrays.asList(200, 201, 202))

WS.verifyElementPropertyValue(response, 'name', 'Aldiransyah Rizky')
WS.verifyElementPropertyValue(response, 'username', 'aldirarizkyp')
WS.verifyElementPropertyValue(response, 'email', 'aldira@gmail.com')
WS.verifyElementPropertyValue(response, 'address.street', 'Patal Senayan')
WS.verifyElementPropertyValue(response, 'address.suite', 'Home')
WS.verifyElementPropertyValue(response, 'address.city', 'Jakarta')
WS.verifyElementPropertyValue(response, 'address.zipcode', '12110')
WS.verifyElementPropertyValue(response, 'address.geo.lat', '-37=0.321')
WS.verifyElementPropertyValue(response, 'address.geo.lng', '70.2346')
WS.verifyElementPropertyValue(response, 'phone', '086767675867')
WS.verifyElementPropertyValue(response, 'website', 'aldraaa.id')
WS.verifyElementPropertyValue(response, 'company.name', 'PT Aldira')
WS.verifyElementPropertyValue(response, 'company.catchPhrase', 'Multi-layered client-server neural-net')
WS.verifyElementPropertyValue(response, 'company.bs', 'harness real-time e-markets')
WS.verifyElementPropertyValue(response, 'id', '11')


</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
