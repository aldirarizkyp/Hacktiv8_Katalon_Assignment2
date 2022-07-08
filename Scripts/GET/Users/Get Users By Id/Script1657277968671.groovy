import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys
import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
response = WS.sendRequest(findTestObject('GET Objects/Users Objects/GET Users By Id'))

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

WS.verifyElementPropertyValue(response, 'company.bs', 'e-enable strategic applications')

