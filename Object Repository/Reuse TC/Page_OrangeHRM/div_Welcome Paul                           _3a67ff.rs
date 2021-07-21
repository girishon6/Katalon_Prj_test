<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>div_Welcome Paul                           _3a67ff</name>
   <tag></tag>
   <elementGuidId>ab3fde0f-cbc8-4ef4-bb40-32a5ad2ee6eb</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>//div[@id='branding']</value>
      </entry>
      <entry>
         <key>CSS</key>
         <value>#branding</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>div</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>id</name>
      <type>Main</type>
      <value>branding</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
                
                Welcome Paul
                


    
        
        
    




    

        
        
            
                
                                    
                No new notifications
                
                                    
            
        
    

    
        
            
                
                                    
            

        

        
            


        

        
            

        
    




    
        
            

        
            
        

    



    
        
            Success!        
        
            Successfully Saved        
        
            Successfully Shared        
        
            Successfully Deleted        

    




    
        
            
        
    





    
        People who shared this post    
    
        
            
        

    





    
        People who like this post    
    
        
            
        

    




    var getAccessUrl = '/index.php/buzz/getLogedToBuzz';
    var loginpageURL = '/index.php/auth/login';

    // buzzCommon.js
    var viewLikedEmployees = '/index.php/buzz/viewLikedEmployees';
    var addBuzzCommentURL = '/index.php/buzz/addNewComment';
    var shareShareURL = '/index.php/buzz/shareAPost';
    var getLikedEmployeeListURL = '/index.php/buzz/getLikedEmployeeList';

    // buzzNew.js
    var shareLikeURL = '/index.php/buzz/likeOnShare';
    var commentLikeURL = '/index.php/buzz/likeOnComment';
    var getSharedEmployeeListURL = '/index.php/buzz/getSharedEmployeeList';

    // viewNotificationComponent.js
    var viewMoreShare = '/index.php/buzz/viewShare';
    var buzzURL = &quot;/index.php/buzz/viewBuzz&quot;;
    var ClearNotificationURL = &quot;/index.php/buzz/clearNotificationAjax&quot;;
    var ClickOnNotificationIconURL = &quot;/index.php/buzz/clickOnNotificationIconAjax&quot;;
    var lang_NoNewNotifications = 'No new notifications';
    var lang_NotificationClearFailed = 'Failed to clear notifications';

                

    
        
    

                                    
                        
                    
                    
                        
                    
                                
                    var marketplaceURL = &quot;/index.php/marketPlace/ohrmAddons&quot;;
                    var SubscriberURL = &quot;/index.php/pim/subscriber&quot;;
                
                
                    
                        
    About

    
        
            ×
            About
        
        
            
                
                    
                        
                            Company Name: OrangeHRM
                        
                        
                            Version: Orangehrm OS 4.8
                        
                        
                            Active Employees: 40
                        
                        
                            Employees Terminated: 0
                        
                    
                
                
                        
    



    jQuery(document).ready(function() {
        
               
        jQuery('#aboutDisplayLink').click(function(event) {
            event.stopImmediatePropagation();
            jQuery('#messageToDisplayAbout').css(
                    'display', 'none');
            jQuery('#displayAbout').modal();
            jQuery('#help-menu.panelContainer').attr('style', 'display:block');
            
            var test = jQuery('.panelContainer');
            jQuery('#help-menu.panelContainer').css(
                    'display', 'block');
             
        });

        jQuery('#heartbeatSubmitBtn').click(function(event) {
            event.stopImmediatePropagation();
            jQuery(this).closest('form').ajaxSubmit(function() {
                jQuery('#messageToDisplayAbout').html('Saved');

                if (jQuery('#register_registration').is(':checked')) {
                    jQuery('#registration-section').css(
                            'display', 'none');
                }
                jQuery('#displayAbout').modal('hide');
                jQuery('#messageToDisplayAbout').css(
                        'display', 'block');
                jQuery('#welcome-menu').css('display','none');
            });
        });

        jQuery('#displayAbout').click(function(event) {
            event.stopImmediatePropagation();
        });
        
        jQuery('#heartbeatCancelBtn').click(function(event) {
            event.stopImmediatePropagation();
             jQuery('#welcome-menu').css('display','none');
                 jQuery('#displayAbout').modal('hide');
        });

    })



                        Support
                     
                        Logout
                    
                
                                


    svg path,
    svg rect{
        fill: #FF6700;
    }
    .svgcl{
        position: relative;
        left: -35px;
        top: -31px;
    }
    


    var inputDatePattern = 'Y-m-d' ;
    var separatorString = 'to';
    $( document ).ready(function() {

        $(&quot;#loader-1&quot;).hide();
        empId = location.href[location.href.length-1];
        dates = $('#startDates').find(&quot;:selected&quot;).text().split(&quot; &quot;+separatorString+&quot; &quot;);
        startDate_timesheet = dates[0]+&quot; 00:00:00&quot;;
        endDate_timesheet   = dates[1]+&quot; 00:00:00&quot;;

        clientId  =     &quot;&quot;;
        clientSecret  = &quot;&quot;;
        clientUrl     = &quot;&quot;;
        successUrl  = &quot;&quot;;
        ajaxURL = &quot;/index.php/buzz/viewBuzz&quot;;
        var timeSheetStatus = $('#timesheet_status').find('h2').text();
        if(timeSheetStatus == 'Status: Approved'){

            $('.syncToggl').hide();
        } else {
            $('.syncToggl').show();
        }

    });

        
    function ajaxSyc() {
        $(&quot;#loader-1&quot;).show();

        $.ajax({
                type: &quot;POST&quot;,
                url: ajaxURL,
                data: {
                    'employee_Id':employeeId,
                    'startTime': startDate_timesheet,
                    'endTime': endDate_timesheet,
                    'timeFormat': inputDatePattern,
                    'timeZone': 'GMT'+formatTimeZone()
                },
                contentType: &quot;application/x-www-form-urlencoded&quot;,

                success: function (msg, status, jqXHR) {

                    $(&quot;#loader-1&quot;).hide();
                    msg = JSON.parse(msg);
                    msgCode = msg.statusCode;
                    if (msgCode != null) {
                        if (msgCode == 101) {
                            displayMessages('error',msg.description );
                        } else if (msgCode == 102) {

                            displayMessages('success', msg.description);
                            setTimeout(function () {
                                location.reload();
                            }, 2000);

                        }
                    } else {
                        showErrorMsg();
                    }

                },
                error: function (XMLHttpRequest, textStatus, errorThrown) {
                    $(&quot;#loader-1&quot;).hide();
                    console.log(errorThrown);
                    showErrorMsg();
                }
            });
    }
    
    function startSyc() {
        $(&quot;#loader-1&quot;).show();

    $.ajax({

        type: &quot;POST&quot;,
        url: clientUrl,


        data: {
            'grant_type': 'client_credentials',
            'client_id': clientId,
            'client_secret': clientSecret
        },
        contentType: &quot;application/x-www-form-urlencoded&quot;,


        success: function (msg, status, jqXHR) {

            try {

                msg = $.parseJSON(jqXHR.responseText);

            } catch (err) {
                console.log(err);
                showErrorMsg();
            }

            $.ajax({
                type: &quot;POST&quot;,
                url: successUrl,
                beforeSend: function (xhr) {

                    xhr.setRequestHeader(&quot;Authorization&quot;, &quot;Bearer &quot; + msg.access_token);
                },

                data: {

                    'employee_Id':employeeId,
                    'startTime': startDate_timesheet,
                    'endTime': endDate_timesheet,
                    'timeFormat': inputDatePattern,
                    'timeZone': 'GMT'+formatTimeZone()
                },
                contentType: &quot;application/x-www-form-urlencoded&quot;,

                success: function (msg, status, jqXHR) {

                    $(&quot;#loader-1&quot;).hide();
                    msgCode = msg.statusCode;
                    if (msgCode != null) {
                        if (msgCode == 101) {
                            displayMessages('error',msg.description );
                        } else if (msgCode == 102) {

                            displayMessages('success', msg.description);
                            setTimeout(function () {
                                location.reload();
                            }, 2000);

                        }
                    } else {
                        showErrorMsg();
                    }

                },
                error: function (XMLHttpRequest, textStatus, errorThrown) {
                    $(&quot;#loader-1&quot;).hide();
                    console.log(errorThrown);
                    showErrorMsg();
                }
            });

        },
        error: function (XMLHttpRequest, textStatus, errorThrown) {
            $(&quot;#loader-1&quot;).hide();
            console.log(errorThrown);
            showErrorMsg();
        }


    });

    }

    function showErrorMsg(){
        displayMessages('error','Unable To Sync With Toggl' );
        setTimeout(function () {
            $('#msgDiv').remove();
        }, 3000);

    }




    
    
        
            
  
      
  
  
        
    

    
        
            ×
            Confirm Toggl Sync
        
        
            Any existing timesheet entry will be overwritten if record for same date is matched. Click ok to continue.
        
        
                            
                        
        
    






            </value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;branding&quot;)</value>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:attributes</name>
      <type>Main</type>
      <value>//div[@id='branding']</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:idRelative</name>
      <type>Main</type>
      <value>//div[@id='wrapper']/div</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Looking'])[1]/following::div[5]</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//body/div/div</value>
   </webElementXpaths>
</WebElementEntity>
