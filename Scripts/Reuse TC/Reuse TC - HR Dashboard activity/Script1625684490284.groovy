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

WebUI.callTestCase(findTestCase('Reuse TC/Reuse Testcase login'), [:], FailureHandling.OPTIONAL)

WebUI.click(findTestObject('Object Repository/Reuse TC/Page_OrangeHRM/div_Welcome Paul                           _3a67ff'))

WebUI.click(findTestObject('Object Repository/Reuse TC/Page_OrangeHRM/a_Users'))

WebUI.setText(findTestObject('Object Repository/Reuse TC/Page_OrangeHRM/input_Username_searchSystemUseruserName'), 'Rishi')

WebUI.click(findTestObject('Object Repository/Reuse TC/Page_OrangeHRM/input_Status_btnAdd'))

WebUI.setText(findTestObject('Object Repository/Reuse TC/Page_OrangeHRM/input__systemUseremployeeNameempName'), 'Ri')

WebUI.click(findTestObject('Object Repository/Reuse TC/Page_OrangeHRM/li_David Morris'))

WebUI.setText(findTestObject('Object Repository/Reuse TC/Page_OrangeHRM/input__systemUseruserName'), 'DMMMM')

WebUI.setEncryptedText(findTestObject('Object Repository/Reuse TC/Page_OrangeHRM/input__systemUserpassword'), 'HKA7rJzgFGhBSAD1t8OXtg==')

WebUI.setEncryptedText(findTestObject('Object Repository/Reuse TC/Page_OrangeHRM/input__systemUserconfirmPassword'), 'HKA7rJzgFGhBSAD1t8OXtg==')

WebUI.click(findTestObject('Object Repository/Reuse TC/Page_OrangeHRM/input__btnSave'))

WebUI.click(findTestObject('Object Repository/Reuse TC/Page_OrangeHRM/a_Welcome Paul'))

WebUI.click(findTestObject('Object Repository/Reuse TC/Page_OrangeHRM/a_Logout'))

