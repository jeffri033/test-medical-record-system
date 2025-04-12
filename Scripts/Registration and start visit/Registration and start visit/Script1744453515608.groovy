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

WebUI.click(findTestObject('Object Repository/Page_Home/i_Register a patient_icon-user'))

WebUI.setText(findTestObject('Object Repository/Page_OpenMRS Electronic Medical Record/input_(required)_givenName'), 'Ujang')

WebUI.setText(findTestObject('Object Repository/Page_OpenMRS Electronic Medical Record/input_Middle_middleName'), 'Setyo')

WebUI.setText(findTestObject('Object Repository/Page_OpenMRS Electronic Medical Record/input_(required)_familyName'), 'Iskandar')

WebUI.click(findTestObject('Object Repository/Page_OpenMRS Electronic Medical Record/span_Gender'))

WebUI.selectOptionByValue(findTestObject('Object Repository/Page_OpenMRS Electronic Medical Record/select_Male                    Female'), 
    'M', true)

WebUI.click(findTestObject('Object Repository/Page_OpenMRS Electronic Medical Record/span_Address'))

WebUI.setText(findTestObject('Object Repository/Page_OpenMRS Electronic Medical Record/input_(required)_birthdateDay'), 
    '01')

WebUI.selectOptionByValue(findTestObject('Object Repository/Page_OpenMRS Electronic Medical Record/select_Select                            Ja_a574ad'), 
    '2', true)

WebUI.setText(findTestObject('Object Repository/Page_OpenMRS Electronic Medical Record/input_(required)_birthdateYear'), 
    '1998')

WebUI.click(findTestObject('Object Repository/Page_OpenMRS Electronic Medical Record/span_Address'))

WebUI.setText(findTestObject('Object Repository/Page_OpenMRS Electronic Medical Record/input_Address_address1'), 'Jl. Melati No 22')

WebUI.setText(findTestObject('Object Repository/Page_OpenMRS Electronic Medical Record/input_CityVillage_cityVillage'), 
    'Bandung')

WebUI.setText(findTestObject('Object Repository/Page_OpenMRS Electronic Medical Record/input_StateProvince_stateProvince'), 
    'West Java')

WebUI.setText(findTestObject('Object Repository/Page_OpenMRS Electronic Medical Record/input_Country_country'), 'Indonesia')

WebUI.click(findTestObject('Object Repository/Page_OpenMRS Electronic Medical Record/input_Country_country'))

WebUI.setText(findTestObject('Object Repository/Page_OpenMRS Electronic Medical Record/input_Postal Code_postalCode'), '124356')

WebUI.click(findTestObject('Object Repository/Page_OpenMRS Electronic Medical Record/span_Phone Number'))

WebUI.setText(findTestObject('Object Repository/Page_OpenMRS Electronic Medical Record/input_concat(What, , s the patient phone nu_867169'), 
    '089945645654')

WebUI.click(findTestObject('Object Repository/Page_OpenMRS Electronic Medical Record/span_Relatives'))

WebUI.selectOptionByValue(findTestObject('Object Repository/Page_OpenMRS Electronic Medical Record/select_Select Relationship Type            _961917'), 
    '8d91a210-c2cc-11de-8d13-0010c6dffd0f-A', true)

WebUI.setText(findTestObject('Object Repository/Page_OpenMRS Electronic Medical Record/input_Who is the patient related to_person-_c8514e'), 
    'Iskandar')

WebUI.click(findTestObject('Object Repository/Page_OpenMRS Electronic Medical Record/form_DemographicsNameGenderBirthdateContact_bc94bc'))

WebUI.click(findTestObject('Object Repository/Page_OpenMRS Electronic Medical Record/span_Confirm'))

WebUI.click(findTestObject('Object Repository/Page_OpenMRS Electronic Medical Record/input_Confirm submission_submit'))

WebUI.click(findTestObject('Object Repository/Page_OpenMRS Electronic Medical Record/div_Start Visit'))

WebUI.click(findTestObject('Object Repository/Page_OpenMRS Electronic Medical Record/button_Confirm'))

