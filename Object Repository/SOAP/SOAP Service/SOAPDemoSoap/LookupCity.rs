<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>LookupCity</name>
   <tag></tag>
   <elementGuidId>f42772c9-8818-40d7-bda6-639e46257a6b</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>SOAPAction</name>
      <type>Main</type>
      <value>http://tempuri.org/SOAP.Demo.LookupCity</value>
      <webElementGuid>4770cb99-bb18-4092-a6d0-f4b69cd221c7</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>text/xml; charset=utf-8</value>
      <webElementGuid>5e55929c-c08a-4f27-a84b-c4c77c209b09</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.6.6</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;soapenv:Envelope xmlns:soapenv=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot; xmlns:tem=&quot;http://tempuri.org&quot;>
   &lt;soapenv:Header/>
   &lt;soapenv:Body>
      &lt;tem:LookupCity>
         &lt;tem:zip>gero et&lt;/tem:zip>
      &lt;/tem:LookupCity>
   &lt;/soapenv:Body>
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceEndpoint>https://www.crcind.com:443/csp/samples/SOAP.Demo.cls</soapServiceEndpoint>
   <soapServiceFunction>LookupCity</soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>false</useServiceInfoFromWsdl>
   <wsdlAddress>https://www.crcind.com/csp/samples/SOAP.Demo.CLS?WSDL=1</wsdlAddress>
</WebServiceRequestEntity>