<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>form_DemographicsNameGenderBirthdateContact_bc94bc</name>
   <tag></tag>
   <elementGuidId>f450a601-6033-4160-a871-863d887ec1cb</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>//form[@id='registration']</value>
      </entry>
      <entry>
         <key>CSS</key>
         <value>#registration</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <smartLocatorCollection>
      <entry>
         <key>SMART_LOCATOR</key>
         <value>#registration</value>
      </entry>
   </smartLocatorCollection>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>form</value>
      <webElementGuid>1454af1b-8a0b-4cdb-b701-59a1925788e1</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>simple-form-ui</value>
      <webElementGuid>bbdf9cc5-eec9-4f03-953f-dfff881b8172</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>id</name>
      <type>Main</type>
      <value>registration</value>
      <webElementGuid>ea1d0e5e-bdfa-4f27-86ca-4f9492a2b3c1</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>method</name>
      <type>Main</type>
      <value>POST</value>
      <webElementGuid>da619a23-e165-48f0-a7b4-877336e1c54e</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>DemographicsNameGenderBirthdateContact InfoAddressPhone NumberRelationshipsRelativesConfirm

        

        
        

            
                

                  
                  

                    
                    

                        

                            Name
                            
                                What's the patient's name?

                                
                                



    
        Given (required)
    

    

    
    





    jq(function() {
        jq(&quot;#fr6060-field&quot; ).autocomplete({
            source: function( request, response ) {
                jq.ajax({
                    url: &quot;/openmrs/registrationapp/personName/getSimilarNames.action?successUrl=%2Fopenmrs%2Fregistrationapp%2FregisterPatient.page%3FappId%3Dreferenceapplication.registrationapp.registerPatient%26&quot;,
                    dataType: &quot;json&quot;,
                    data: {
                        'searchPhrase': request.term,
                        'formFieldName': 'givenName'
                    },
                    success: function( data ) {
                        response( jq.map( data.names, function( item ) {
                            return {
                                label: item,
                                value: item
                            }
                        }));
                    }
                });
            },
            minLength: 1
        });
    });




                                
                                



    
        Middle 
    

    No search results.

    
    





    jq(function() {
        jq(&quot;#fr7373-field&quot; ).autocomplete({
            source: function( request, response ) {
                jq.ajax({
                    url: &quot;/openmrs/registrationapp/personName/getSimilarNames.action?successUrl=%2Fopenmrs%2Fregistrationapp%2FregisterPatient.page%3FappId%3Dreferenceapplication.registrationapp.registerPatient%26&quot;,
                    dataType: &quot;json&quot;,
                    data: {
                        'searchPhrase': request.term,
                        'formFieldName': 'middleName'
                    },
                    success: function( data ) {
                        response( jq.map( data.names, function( item ) {
                            return {
                                label: item,
                                value: item
                            }
                        }));
                    }
                });
            },
            minLength: 1
        });
    });




                                
                                



    
        Family Name (required)
    

    No search results.

    
    





    jq(function() {
        jq(&quot;#fr9922-field&quot; ).autocomplete({
            source: function( request, response ) {
                jq.ajax({
                    url: &quot;/openmrs/registrationapp/personName/getSimilarNames.action?successUrl=%2Fopenmrs%2Fregistrationapp%2FregisterPatient.page%3FappId%3Dreferenceapplication.registrationapp.registerPatient%26&quot;,
                    dataType: &quot;json&quot;,
                    data: {
                        'searchPhrase': request.term,
                        'formFieldName': 'familyName'
                    },
                    success: function( data ) {
                        response( jq.map( data.names, function( item ) {
                            return {
                                label: item,
                                value: item
                            }
                        }));
                    }
                });
            },
            minLength: 1
        });
    });




                                
                            

                            
                            
		                    

                            

                            
                            
                                
                                
                                    
                                    Unidentified Patient
                                
                            

                            


                            
                        

                        

                            Gender

                            



    
        What's the patient's gender? (required)
    

    

    
        
        
            Male
        
            Female
        
    

    




    var viewModel = viewModel || {};
    viewModel.validations = viewModel.validations || [];
    viewModel.gender = ko.observable();

    

        
        viewModel.validations.push(function() {
            return jq('#gender-field').is(':disabled') || (viewModel.gender() ? true : false);
        });
        
    

    



                            
                            
                        

                        

                            Birthdate

                            


    var birthdateDay = '';
    var birthdateMonth = '';
    var birthdateYear = '';

    jQuery(document).ready(function(){
        

        _.each(jQuery('.date-component'), function(dateElement){
            jQuery(dateElement).blur(function(){
                var elementValue = '';
                if(this.value &amp;&amp; jQuery.trim(this.value).length > 0)
                    elementValue = jQuery.trim(this.value);

                if(this.id == 'birthdateDay-field'){
                    if(elementValue.length > 0)
                        birthdateDay = elementValue;
                    else
                        birthdateDay = '';
                }else if(this.id == 'birthdateMonth-field'){
                    if(elementValue.length > 0)
                        birthdateMonth = elementValue;
                    else
                        birthdateMonth = '';
                }else if (this.id == 'birthdateYear-field'){
                    if(elementValue.length > 0)
                        birthdateYear = elementValue;
                    else
                        birthdateYear = '';
                }

                if (birthdateYear != '' &amp;&amp; birthdateMonth != '' &amp;&amp; birthdateDay != '') {
                    jQuery('#birthdate-value').val(birthdateYear+&quot;-&quot;+birthdateMonth+&quot;-&quot;+birthdateDay);
                } else {
                    jQuery('#birthdate-value').val('');
                    jQuery('#birthdateYears-field').prop('disabled', false);
                    jQuery('#birthdateMonths-field').prop('disabled', false);
                }
            });
        });
    });




    
        What's the patient's birth date? (required)
    
    

    


    
        Day (required)
    
    
    
    
    
    


    



    
        Month (required)
    

    

    
        
            Select
        
        
            January
        
            February
        
            March
        
            April
        
            May
        
            June
        
            July
        
            August
        
            September
        
            October
        
            November
        
            December
        
    

    




    var viewModel = viewModel || {};
    viewModel.validations = viewModel.validations || [];
    viewModel.birthdateMonth = ko.observable();

    

        
        viewModel.validations.push(function() {
            return jq('#birthdateMonth-field').is(':disabled') || (viewModel.birthdateMonth() ? true : false);
        });
        
    

    



    


    
        Year (required)
    
    
    
    
    
    


            
    
	
    



    jQuery(function() {
        var yearsField = jQuery('#birthdateYears-field');
        var monthsField = jQuery('#birthdateMonths-field');
        var estimatedField = jQuery('#birthdateEstimated-field');

        var skipField = function() {
            e = jQuery.Event('keydown');
            e.which = 13; // enter
            yearsField.trigger(e);
        };

        var isExactDate = function() {
            var isEstimatedChecked = jQuery('#birthdateEstimated-field').is(':checked');
            return (jQuery('#birthdate-value').val() != '' &amp;&amp; !isEstimatedChecked);
        }

        var toggleEstimatedDate = function() {
            if(isExactDate()) {
                yearsField.val('');
                yearsField.prop('disabled', true);
                monthsField.val('');
                monthsField.prop('disabled', true);
            } else {
                yearsField.prop('disabled', false);
                monthsField.prop('disabled', false);
            }
        };

        var clearExactDateFields = function() {
            birthdateDay = '';
            birthdateMonth = '';
            birthdateYear = '';
            jQuery('#birthdateDay-field').val('');
            jQuery('#birthdateMonth-field').val('');
            jQuery('#birthdateYear-field').val('');

            if (birthdateYear != '' &amp;&amp; birthdateMonth != '' &amp;&amp; birthdateDay != '') {
                jQuery('#birthdate-value').val(birthdateYear+&quot;-&quot;+birthdateMonth+&quot;-&quot;+birthdateDay);
            } else {
                jQuery('#birthdate-value').val('');
            }
        }

        jQuery(estimatedField).change(function() {
            toggleEstimatedDate();
        });

        jQuery(yearsField).blur(function() {
            var elementValue = '';
            if(this.value &amp;&amp; jQuery.trim(this.value).length > 0)
                elementValue = jQuery.trim(this.value);

            if(elementValue.length > 0) {
                birthdateYears = elementValue;
                clearExactDateFields();
            }else {
                birthdateYears = '';
            }

        });

        jQuery(monthsField).blur(function() {
            var elementValue = '';
            if(this.value &amp;&amp; jQuery.trim(this.value).length > 0)
                elementValue = jQuery.trim(this.value);

            if(elementValue.length > 0) {
                birthdateMonths = elementValue;
                clearExactDateFields();
            }else {
                birthdateMonths = '';
            }

        });

        jQuery(yearsField).focus(toggleEstimatedDate);
        jQuery(monthsField).focus(toggleEstimatedDate);
    });

 
Or

    


    
        Estimated Years (required)
    
    
    
    
     year(s)
    



    


    
        Estimated Months (required)
    
    
    
    
     month(s)
    







                        
                    

                    
                    

                    
                  
            
        

            
                

                  
                  

                    
                    

                    
                    

                    
	                        
	                            Address

	                            
	                                
	                            
	                            

	                            
		                            

    
        What is the patient's address? 
    


        
            
        
            
            	 
					
	        			Address
	        		
	        		
                        
                        
                    
	        	
	        	    
            
        

        
            
        
            
            	 
					
	        			Address 2
	        		
	        		
                        
                        
                    
	        	
	        	    
            
        

        
            
        
            
            	 
					
	        			City/Village
	        		
	        		
                        
                        
                    
	        	
	        	  
            
        
            
        
            
            	 
					
	        			State/Province
	        		
	        		
                        
                        
                    
	        	
	        	  
            
        
            
        
            
            	 
					
	        			Country
	        		
	        		
                        
                        
                    
	        	
	        	  
            
        
            
        
            
            	 
					
	        			Postal Code
	        		
	        		
                        
                        
                    
	        	
	        	    
            
        


		                        
	                        
	                    
                    
	                        
	                            Phone Number

	                            
	                            

	                            
		                            


    
        What's the patient phone number? 
    
    
    
    
    
    


		                        
	                        
	                    
                    
                  
            
        

            
                

                  
                  

                    
                    

                    
                    

                    
	                        
	                            Relatives

	                            
	                            
	                                    Who is the patient related to?
	                            

	                            
		                            



    
        
            
                Select Relationship Type
                
                    
                        
                        Doctor
                    
                
                    
                        
                        Sibling
                    
                
                    
                        
                        Parent
                    
                
                    
                        
                        Aunt/Uncle
                    
                
                    
                        
                        Supervisor
                    
                
                
                    
                        
                            
                            Patient
                        
                    
                
                    
                
                    
                        
                            
                            Child
                        
                    
                
                    
                        
                            
                            Niece/Nephew
                        
                    
                
                    
                        
                            
                            Supervisee
                        
                    
                
            
        

        
            
    
        Jojo Adhi Sunandar
    
        Jono Sugiarto Sunandar
    
        Joko Sunarto Sunandar
    

            
        

        
            
                
            
            
                
            
        
    

		                        
	                        
	                    
                    
                  
            
        

        

        
            
            
            
            
            
                There seems to be a patient in the database that exactly matches this one. Please review before confirming:
                
            
            
                Confirm submission?
                
                    
                
                
                    
                
            
        
        </value>
      <webElementGuid>2113992b-deb6-4890-b31f-e76ce24708f5</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;registration&quot;)</value>
      <webElementGuid>2b999022-7e02-4a54-ae9b-d752b7056014</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:attributes</name>
      <type>Main</type>
      <value>//form[@id='registration']</value>
      <webElementGuid>af5ac234-cb78-4694-9cb2-0d700a2f079f</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:idRelative</name>
      <type>Main</type>
      <value>//div[@id='content']/form</value>
      <webElementGuid>bfc95c42-73d2-40ba-9283-67fab3d1ca33</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Import and Open'])[1]/following::form[1]</value>
      <webElementGuid>3876aeb0-d428-4550-85c2-e631ad45aa93</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Open'])[4]/following::form[1]</value>
      <webElementGuid>6b597f56-e051-4ea5-8844-5de3a0f35bce</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//form</value>
      <webElementGuid>f5beab8a-fee1-45d3-9d96-73b05b16ca8f</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//form[@id = 'registration' and (text() = concat(&quot;DemographicsNameGenderBirthdateContact InfoAddressPhone NumberRelationshipsRelativesConfirm

        

        
        

            
                

                  
                  

                    
                    

                        

                            Name
                            
                                What&quot; , &quot;'&quot; , &quot;s the patient&quot; , &quot;'&quot; , &quot;s name?

                                
                                



    
        Given (required)
    

    

    
    





    jq(function() {
        jq(&quot;#fr6060-field&quot; ).autocomplete({
            source: function( request, response ) {
                jq.ajax({
                    url: &quot;/openmrs/registrationapp/personName/getSimilarNames.action?successUrl=%2Fopenmrs%2Fregistrationapp%2FregisterPatient.page%3FappId%3Dreferenceapplication.registrationapp.registerPatient%26&quot;,
                    dataType: &quot;json&quot;,
                    data: {
                        &quot; , &quot;'&quot; , &quot;searchPhrase&quot; , &quot;'&quot; , &quot;: request.term,
                        &quot; , &quot;'&quot; , &quot;formFieldName&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;givenName&quot; , &quot;'&quot; , &quot;
                    },
                    success: function( data ) {
                        response( jq.map( data.names, function( item ) {
                            return {
                                label: item,
                                value: item
                            }
                        }));
                    }
                });
            },
            minLength: 1
        });
    });




                                
                                



    
        Middle 
    

    No search results.

    
    





    jq(function() {
        jq(&quot;#fr7373-field&quot; ).autocomplete({
            source: function( request, response ) {
                jq.ajax({
                    url: &quot;/openmrs/registrationapp/personName/getSimilarNames.action?successUrl=%2Fopenmrs%2Fregistrationapp%2FregisterPatient.page%3FappId%3Dreferenceapplication.registrationapp.registerPatient%26&quot;,
                    dataType: &quot;json&quot;,
                    data: {
                        &quot; , &quot;'&quot; , &quot;searchPhrase&quot; , &quot;'&quot; , &quot;: request.term,
                        &quot; , &quot;'&quot; , &quot;formFieldName&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;middleName&quot; , &quot;'&quot; , &quot;
                    },
                    success: function( data ) {
                        response( jq.map( data.names, function( item ) {
                            return {
                                label: item,
                                value: item
                            }
                        }));
                    }
                });
            },
            minLength: 1
        });
    });




                                
                                



    
        Family Name (required)
    

    No search results.

    
    





    jq(function() {
        jq(&quot;#fr9922-field&quot; ).autocomplete({
            source: function( request, response ) {
                jq.ajax({
                    url: &quot;/openmrs/registrationapp/personName/getSimilarNames.action?successUrl=%2Fopenmrs%2Fregistrationapp%2FregisterPatient.page%3FappId%3Dreferenceapplication.registrationapp.registerPatient%26&quot;,
                    dataType: &quot;json&quot;,
                    data: {
                        &quot; , &quot;'&quot; , &quot;searchPhrase&quot; , &quot;'&quot; , &quot;: request.term,
                        &quot; , &quot;'&quot; , &quot;formFieldName&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;familyName&quot; , &quot;'&quot; , &quot;
                    },
                    success: function( data ) {
                        response( jq.map( data.names, function( item ) {
                            return {
                                label: item,
                                value: item
                            }
                        }));
                    }
                });
            },
            minLength: 1
        });
    });




                                
                            

                            
                            
		                    

                            

                            
                            
                                
                                
                                    
                                    Unidentified Patient
                                
                            

                            


                            
                        

                        

                            Gender

                            



    
        What&quot; , &quot;'&quot; , &quot;s the patient&quot; , &quot;'&quot; , &quot;s gender? (required)
    

    

    
        
        
            Male
        
            Female
        
    

    




    var viewModel = viewModel || {};
    viewModel.validations = viewModel.validations || [];
    viewModel.gender = ko.observable();

    

        
        viewModel.validations.push(function() {
            return jq(&quot; , &quot;'&quot; , &quot;#gender-field&quot; , &quot;'&quot; , &quot;).is(&quot; , &quot;'&quot; , &quot;:disabled&quot; , &quot;'&quot; , &quot;) || (viewModel.gender() ? true : false);
        });
        
    

    



                            
                            
                        

                        

                            Birthdate

                            


    var birthdateDay = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
    var birthdateMonth = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
    var birthdateYear = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;

    jQuery(document).ready(function(){
        

        _.each(jQuery(&quot; , &quot;'&quot; , &quot;.date-component&quot; , &quot;'&quot; , &quot;), function(dateElement){
            jQuery(dateElement).blur(function(){
                var elementValue = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
                if(this.value &amp;&amp; jQuery.trim(this.value).length > 0)
                    elementValue = jQuery.trim(this.value);

                if(this.id == &quot; , &quot;'&quot; , &quot;birthdateDay-field&quot; , &quot;'&quot; , &quot;){
                    if(elementValue.length > 0)
                        birthdateDay = elementValue;
                    else
                        birthdateDay = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
                }else if(this.id == &quot; , &quot;'&quot; , &quot;birthdateMonth-field&quot; , &quot;'&quot; , &quot;){
                    if(elementValue.length > 0)
                        birthdateMonth = elementValue;
                    else
                        birthdateMonth = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
                }else if (this.id == &quot; , &quot;'&quot; , &quot;birthdateYear-field&quot; , &quot;'&quot; , &quot;){
                    if(elementValue.length > 0)
                        birthdateYear = elementValue;
                    else
                        birthdateYear = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
                }

                if (birthdateYear != &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot; &amp;&amp; birthdateMonth != &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot; &amp;&amp; birthdateDay != &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;) {
                    jQuery(&quot; , &quot;'&quot; , &quot;#birthdate-value&quot; , &quot;'&quot; , &quot;).val(birthdateYear+&quot;-&quot;+birthdateMonth+&quot;-&quot;+birthdateDay);
                } else {
                    jQuery(&quot; , &quot;'&quot; , &quot;#birthdate-value&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
                    jQuery(&quot; , &quot;'&quot; , &quot;#birthdateYears-field&quot; , &quot;'&quot; , &quot;).prop(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, false);
                    jQuery(&quot; , &quot;'&quot; , &quot;#birthdateMonths-field&quot; , &quot;'&quot; , &quot;).prop(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, false);
                }
            });
        });
    });




    
        What&quot; , &quot;'&quot; , &quot;s the patient&quot; , &quot;'&quot; , &quot;s birth date? (required)
    
    

    


    
        Day (required)
    
    
    
    
    
    


    



    
        Month (required)
    

    

    
        
            Select
        
        
            January
        
            February
        
            March
        
            April
        
            May
        
            June
        
            July
        
            August
        
            September
        
            October
        
            November
        
            December
        
    

    




    var viewModel = viewModel || {};
    viewModel.validations = viewModel.validations || [];
    viewModel.birthdateMonth = ko.observable();

    

        
        viewModel.validations.push(function() {
            return jq(&quot; , &quot;'&quot; , &quot;#birthdateMonth-field&quot; , &quot;'&quot; , &quot;).is(&quot; , &quot;'&quot; , &quot;:disabled&quot; , &quot;'&quot; , &quot;) || (viewModel.birthdateMonth() ? true : false);
        });
        
    

    



    


    
        Year (required)
    
    
    
    
    
    


            
    
	
    



    jQuery(function() {
        var yearsField = jQuery(&quot; , &quot;'&quot; , &quot;#birthdateYears-field&quot; , &quot;'&quot; , &quot;);
        var monthsField = jQuery(&quot; , &quot;'&quot; , &quot;#birthdateMonths-field&quot; , &quot;'&quot; , &quot;);
        var estimatedField = jQuery(&quot; , &quot;'&quot; , &quot;#birthdateEstimated-field&quot; , &quot;'&quot; , &quot;);

        var skipField = function() {
            e = jQuery.Event(&quot; , &quot;'&quot; , &quot;keydown&quot; , &quot;'&quot; , &quot;);
            e.which = 13; // enter
            yearsField.trigger(e);
        };

        var isExactDate = function() {
            var isEstimatedChecked = jQuery(&quot; , &quot;'&quot; , &quot;#birthdateEstimated-field&quot; , &quot;'&quot; , &quot;).is(&quot; , &quot;'&quot; , &quot;:checked&quot; , &quot;'&quot; , &quot;);
            return (jQuery(&quot; , &quot;'&quot; , &quot;#birthdate-value&quot; , &quot;'&quot; , &quot;).val() != &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot; &amp;&amp; !isEstimatedChecked);
        }

        var toggleEstimatedDate = function() {
            if(isExactDate()) {
                yearsField.val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
                yearsField.prop(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, true);
                monthsField.val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
                monthsField.prop(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, true);
            } else {
                yearsField.prop(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, false);
                monthsField.prop(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, false);
            }
        };

        var clearExactDateFields = function() {
            birthdateDay = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
            birthdateMonth = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
            birthdateYear = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
            jQuery(&quot; , &quot;'&quot; , &quot;#birthdateDay-field&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
            jQuery(&quot; , &quot;'&quot; , &quot;#birthdateMonth-field&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
            jQuery(&quot; , &quot;'&quot; , &quot;#birthdateYear-field&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);

            if (birthdateYear != &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot; &amp;&amp; birthdateMonth != &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot; &amp;&amp; birthdateDay != &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;) {
                jQuery(&quot; , &quot;'&quot; , &quot;#birthdate-value&quot; , &quot;'&quot; , &quot;).val(birthdateYear+&quot;-&quot;+birthdateMonth+&quot;-&quot;+birthdateDay);
            } else {
                jQuery(&quot; , &quot;'&quot; , &quot;#birthdate-value&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
            }
        }

        jQuery(estimatedField).change(function() {
            toggleEstimatedDate();
        });

        jQuery(yearsField).blur(function() {
            var elementValue = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
            if(this.value &amp;&amp; jQuery.trim(this.value).length > 0)
                elementValue = jQuery.trim(this.value);

            if(elementValue.length > 0) {
                birthdateYears = elementValue;
                clearExactDateFields();
            }else {
                birthdateYears = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
            }

        });

        jQuery(monthsField).blur(function() {
            var elementValue = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
            if(this.value &amp;&amp; jQuery.trim(this.value).length > 0)
                elementValue = jQuery.trim(this.value);

            if(elementValue.length > 0) {
                birthdateMonths = elementValue;
                clearExactDateFields();
            }else {
                birthdateMonths = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
            }

        });

        jQuery(yearsField).focus(toggleEstimatedDate);
        jQuery(monthsField).focus(toggleEstimatedDate);
    });

 
Or

    


    
        Estimated Years (required)
    
    
    
    
     year(s)
    



    


    
        Estimated Months (required)
    
    
    
    
     month(s)
    







                        
                    

                    
                    

                    
                  
            
        

            
                

                  
                  

                    
                    

                    
                    

                    
	                        
	                            Address

	                            
	                                
	                            
	                            

	                            
		                            

    
        What is the patient&quot; , &quot;'&quot; , &quot;s address? 
    


        
            
        
            
            	 
					
	        			Address
	        		
	        		
                        
                        
                    
	        	
	        	    
            
        

        
            
        
            
            	 
					
	        			Address 2
	        		
	        		
                        
                        
                    
	        	
	        	    
            
        

        
            
        
            
            	 
					
	        			City/Village
	        		
	        		
                        
                        
                    
	        	
	        	  
            
        
            
        
            
            	 
					
	        			State/Province
	        		
	        		
                        
                        
                    
	        	
	        	  
            
        
            
        
            
            	 
					
	        			Country
	        		
	        		
                        
                        
                    
	        	
	        	  
            
        
            
        
            
            	 
					
	        			Postal Code
	        		
	        		
                        
                        
                    
	        	
	        	    
            
        


		                        
	                        
	                    
                    
	                        
	                            Phone Number

	                            
	                            

	                            
		                            


    
        What&quot; , &quot;'&quot; , &quot;s the patient phone number? 
    
    
    
    
    
    


		                        
	                        
	                    
                    
                  
            
        

            
                

                  
                  

                    
                    

                    
                    

                    
	                        
	                            Relatives

	                            
	                            
	                                    Who is the patient related to?
	                            

	                            
		                            



    
        
            
                Select Relationship Type
                
                    
                        
                        Doctor
                    
                
                    
                        
                        Sibling
                    
                
                    
                        
                        Parent
                    
                
                    
                        
                        Aunt/Uncle
                    
                
                    
                        
                        Supervisor
                    
                
                
                    
                        
                            
                            Patient
                        
                    
                
                    
                
                    
                        
                            
                            Child
                        
                    
                
                    
                        
                            
                            Niece/Nephew
                        
                    
                
                    
                        
                            
                            Supervisee
                        
                    
                
            
        

        
            
    
        Jojo Adhi Sunandar
    
        Jono Sugiarto Sunandar
    
        Joko Sunarto Sunandar
    

            
        

        
            
                
            
            
                
            
        
    

		                        
	                        
	                    
                    
                  
            
        

        

        
            
            
            
            
            
                There seems to be a patient in the database that exactly matches this one. Please review before confirming:
                
            
            
                Confirm submission?
                
                    
                
                
                    
                
            
        
        &quot;) or . = concat(&quot;DemographicsNameGenderBirthdateContact InfoAddressPhone NumberRelationshipsRelativesConfirm

        

        
        

            
                

                  
                  

                    
                    

                        

                            Name
                            
                                What&quot; , &quot;'&quot; , &quot;s the patient&quot; , &quot;'&quot; , &quot;s name?

                                
                                



    
        Given (required)
    

    

    
    





    jq(function() {
        jq(&quot;#fr6060-field&quot; ).autocomplete({
            source: function( request, response ) {
                jq.ajax({
                    url: &quot;/openmrs/registrationapp/personName/getSimilarNames.action?successUrl=%2Fopenmrs%2Fregistrationapp%2FregisterPatient.page%3FappId%3Dreferenceapplication.registrationapp.registerPatient%26&quot;,
                    dataType: &quot;json&quot;,
                    data: {
                        &quot; , &quot;'&quot; , &quot;searchPhrase&quot; , &quot;'&quot; , &quot;: request.term,
                        &quot; , &quot;'&quot; , &quot;formFieldName&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;givenName&quot; , &quot;'&quot; , &quot;
                    },
                    success: function( data ) {
                        response( jq.map( data.names, function( item ) {
                            return {
                                label: item,
                                value: item
                            }
                        }));
                    }
                });
            },
            minLength: 1
        });
    });




                                
                                



    
        Middle 
    

    No search results.

    
    





    jq(function() {
        jq(&quot;#fr7373-field&quot; ).autocomplete({
            source: function( request, response ) {
                jq.ajax({
                    url: &quot;/openmrs/registrationapp/personName/getSimilarNames.action?successUrl=%2Fopenmrs%2Fregistrationapp%2FregisterPatient.page%3FappId%3Dreferenceapplication.registrationapp.registerPatient%26&quot;,
                    dataType: &quot;json&quot;,
                    data: {
                        &quot; , &quot;'&quot; , &quot;searchPhrase&quot; , &quot;'&quot; , &quot;: request.term,
                        &quot; , &quot;'&quot; , &quot;formFieldName&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;middleName&quot; , &quot;'&quot; , &quot;
                    },
                    success: function( data ) {
                        response( jq.map( data.names, function( item ) {
                            return {
                                label: item,
                                value: item
                            }
                        }));
                    }
                });
            },
            minLength: 1
        });
    });




                                
                                



    
        Family Name (required)
    

    No search results.

    
    





    jq(function() {
        jq(&quot;#fr9922-field&quot; ).autocomplete({
            source: function( request, response ) {
                jq.ajax({
                    url: &quot;/openmrs/registrationapp/personName/getSimilarNames.action?successUrl=%2Fopenmrs%2Fregistrationapp%2FregisterPatient.page%3FappId%3Dreferenceapplication.registrationapp.registerPatient%26&quot;,
                    dataType: &quot;json&quot;,
                    data: {
                        &quot; , &quot;'&quot; , &quot;searchPhrase&quot; , &quot;'&quot; , &quot;: request.term,
                        &quot; , &quot;'&quot; , &quot;formFieldName&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;familyName&quot; , &quot;'&quot; , &quot;
                    },
                    success: function( data ) {
                        response( jq.map( data.names, function( item ) {
                            return {
                                label: item,
                                value: item
                            }
                        }));
                    }
                });
            },
            minLength: 1
        });
    });




                                
                            

                            
                            
		                    

                            

                            
                            
                                
                                
                                    
                                    Unidentified Patient
                                
                            

                            


                            
                        

                        

                            Gender

                            



    
        What&quot; , &quot;'&quot; , &quot;s the patient&quot; , &quot;'&quot; , &quot;s gender? (required)
    

    

    
        
        
            Male
        
            Female
        
    

    




    var viewModel = viewModel || {};
    viewModel.validations = viewModel.validations || [];
    viewModel.gender = ko.observable();

    

        
        viewModel.validations.push(function() {
            return jq(&quot; , &quot;'&quot; , &quot;#gender-field&quot; , &quot;'&quot; , &quot;).is(&quot; , &quot;'&quot; , &quot;:disabled&quot; , &quot;'&quot; , &quot;) || (viewModel.gender() ? true : false);
        });
        
    

    



                            
                            
                        

                        

                            Birthdate

                            


    var birthdateDay = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
    var birthdateMonth = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
    var birthdateYear = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;

    jQuery(document).ready(function(){
        

        _.each(jQuery(&quot; , &quot;'&quot; , &quot;.date-component&quot; , &quot;'&quot; , &quot;), function(dateElement){
            jQuery(dateElement).blur(function(){
                var elementValue = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
                if(this.value &amp;&amp; jQuery.trim(this.value).length > 0)
                    elementValue = jQuery.trim(this.value);

                if(this.id == &quot; , &quot;'&quot; , &quot;birthdateDay-field&quot; , &quot;'&quot; , &quot;){
                    if(elementValue.length > 0)
                        birthdateDay = elementValue;
                    else
                        birthdateDay = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
                }else if(this.id == &quot; , &quot;'&quot; , &quot;birthdateMonth-field&quot; , &quot;'&quot; , &quot;){
                    if(elementValue.length > 0)
                        birthdateMonth = elementValue;
                    else
                        birthdateMonth = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
                }else if (this.id == &quot; , &quot;'&quot; , &quot;birthdateYear-field&quot; , &quot;'&quot; , &quot;){
                    if(elementValue.length > 0)
                        birthdateYear = elementValue;
                    else
                        birthdateYear = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
                }

                if (birthdateYear != &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot; &amp;&amp; birthdateMonth != &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot; &amp;&amp; birthdateDay != &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;) {
                    jQuery(&quot; , &quot;'&quot; , &quot;#birthdate-value&quot; , &quot;'&quot; , &quot;).val(birthdateYear+&quot;-&quot;+birthdateMonth+&quot;-&quot;+birthdateDay);
                } else {
                    jQuery(&quot; , &quot;'&quot; , &quot;#birthdate-value&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
                    jQuery(&quot; , &quot;'&quot; , &quot;#birthdateYears-field&quot; , &quot;'&quot; , &quot;).prop(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, false);
                    jQuery(&quot; , &quot;'&quot; , &quot;#birthdateMonths-field&quot; , &quot;'&quot; , &quot;).prop(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, false);
                }
            });
        });
    });




    
        What&quot; , &quot;'&quot; , &quot;s the patient&quot; , &quot;'&quot; , &quot;s birth date? (required)
    
    

    


    
        Day (required)
    
    
    
    
    
    


    



    
        Month (required)
    

    

    
        
            Select
        
        
            January
        
            February
        
            March
        
            April
        
            May
        
            June
        
            July
        
            August
        
            September
        
            October
        
            November
        
            December
        
    

    




    var viewModel = viewModel || {};
    viewModel.validations = viewModel.validations || [];
    viewModel.birthdateMonth = ko.observable();

    

        
        viewModel.validations.push(function() {
            return jq(&quot; , &quot;'&quot; , &quot;#birthdateMonth-field&quot; , &quot;'&quot; , &quot;).is(&quot; , &quot;'&quot; , &quot;:disabled&quot; , &quot;'&quot; , &quot;) || (viewModel.birthdateMonth() ? true : false);
        });
        
    

    



    


    
        Year (required)
    
    
    
    
    
    


            
    
	
    



    jQuery(function() {
        var yearsField = jQuery(&quot; , &quot;'&quot; , &quot;#birthdateYears-field&quot; , &quot;'&quot; , &quot;);
        var monthsField = jQuery(&quot; , &quot;'&quot; , &quot;#birthdateMonths-field&quot; , &quot;'&quot; , &quot;);
        var estimatedField = jQuery(&quot; , &quot;'&quot; , &quot;#birthdateEstimated-field&quot; , &quot;'&quot; , &quot;);

        var skipField = function() {
            e = jQuery.Event(&quot; , &quot;'&quot; , &quot;keydown&quot; , &quot;'&quot; , &quot;);
            e.which = 13; // enter
            yearsField.trigger(e);
        };

        var isExactDate = function() {
            var isEstimatedChecked = jQuery(&quot; , &quot;'&quot; , &quot;#birthdateEstimated-field&quot; , &quot;'&quot; , &quot;).is(&quot; , &quot;'&quot; , &quot;:checked&quot; , &quot;'&quot; , &quot;);
            return (jQuery(&quot; , &quot;'&quot; , &quot;#birthdate-value&quot; , &quot;'&quot; , &quot;).val() != &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot; &amp;&amp; !isEstimatedChecked);
        }

        var toggleEstimatedDate = function() {
            if(isExactDate()) {
                yearsField.val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
                yearsField.prop(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, true);
                monthsField.val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
                monthsField.prop(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, true);
            } else {
                yearsField.prop(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, false);
                monthsField.prop(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, false);
            }
        };

        var clearExactDateFields = function() {
            birthdateDay = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
            birthdateMonth = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
            birthdateYear = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
            jQuery(&quot; , &quot;'&quot; , &quot;#birthdateDay-field&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
            jQuery(&quot; , &quot;'&quot; , &quot;#birthdateMonth-field&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
            jQuery(&quot; , &quot;'&quot; , &quot;#birthdateYear-field&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);

            if (birthdateYear != &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot; &amp;&amp; birthdateMonth != &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot; &amp;&amp; birthdateDay != &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;) {
                jQuery(&quot; , &quot;'&quot; , &quot;#birthdate-value&quot; , &quot;'&quot; , &quot;).val(birthdateYear+&quot;-&quot;+birthdateMonth+&quot;-&quot;+birthdateDay);
            } else {
                jQuery(&quot; , &quot;'&quot; , &quot;#birthdate-value&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
            }
        }

        jQuery(estimatedField).change(function() {
            toggleEstimatedDate();
        });

        jQuery(yearsField).blur(function() {
            var elementValue = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
            if(this.value &amp;&amp; jQuery.trim(this.value).length > 0)
                elementValue = jQuery.trim(this.value);

            if(elementValue.length > 0) {
                birthdateYears = elementValue;
                clearExactDateFields();
            }else {
                birthdateYears = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
            }

        });

        jQuery(monthsField).blur(function() {
            var elementValue = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
            if(this.value &amp;&amp; jQuery.trim(this.value).length > 0)
                elementValue = jQuery.trim(this.value);

            if(elementValue.length > 0) {
                birthdateMonths = elementValue;
                clearExactDateFields();
            }else {
                birthdateMonths = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
            }

        });

        jQuery(yearsField).focus(toggleEstimatedDate);
        jQuery(monthsField).focus(toggleEstimatedDate);
    });

 
Or

    


    
        Estimated Years (required)
    
    
    
    
     year(s)
    



    


    
        Estimated Months (required)
    
    
    
    
     month(s)
    







                        
                    

                    
                    

                    
                  
            
        

            
                

                  
                  

                    
                    

                    
                    

                    
	                        
	                            Address

	                            
	                                
	                            
	                            

	                            
		                            

    
        What is the patient&quot; , &quot;'&quot; , &quot;s address? 
    


        
            
        
            
            	 
					
	        			Address
	        		
	        		
                        
                        
                    
	        	
	        	    
            
        

        
            
        
            
            	 
					
	        			Address 2
	        		
	        		
                        
                        
                    
	        	
	        	    
            
        

        
            
        
            
            	 
					
	        			City/Village
	        		
	        		
                        
                        
                    
	        	
	        	  
            
        
            
        
            
            	 
					
	        			State/Province
	        		
	        		
                        
                        
                    
	        	
	        	  
            
        
            
        
            
            	 
					
	        			Country
	        		
	        		
                        
                        
                    
	        	
	        	  
            
        
            
        
            
            	 
					
	        			Postal Code
	        		
	        		
                        
                        
                    
	        	
	        	    
            
        


		                        
	                        
	                    
                    
	                        
	                            Phone Number

	                            
	                            

	                            
		                            


    
        What&quot; , &quot;'&quot; , &quot;s the patient phone number? 
    
    
    
    
    
    


		                        
	                        
	                    
                    
                  
            
        

            
                

                  
                  

                    
                    

                    
                    

                    
	                        
	                            Relatives

	                            
	                            
	                                    Who is the patient related to?
	                            

	                            
		                            



    
        
            
                Select Relationship Type
                
                    
                        
                        Doctor
                    
                
                    
                        
                        Sibling
                    
                
                    
                        
                        Parent
                    
                
                    
                        
                        Aunt/Uncle
                    
                
                    
                        
                        Supervisor
                    
                
                
                    
                        
                            
                            Patient
                        
                    
                
                    
                
                    
                        
                            
                            Child
                        
                    
                
                    
                        
                            
                            Niece/Nephew
                        
                    
                
                    
                        
                            
                            Supervisee
                        
                    
                
            
        

        
            
    
        Jojo Adhi Sunandar
    
        Jono Sugiarto Sunandar
    
        Joko Sunarto Sunandar
    

            
        

        
            
                
            
            
                
            
        
    

		                        
	                        
	                    
                    
                  
            
        

        

        
            
            
            
            
            
                There seems to be a patient in the database that exactly matches this one. Please review before confirming:
                
            
            
                Confirm submission?
                
                    
                
                
                    
                
            
        
        &quot;))]</value>
      <webElementGuid>8f942a7f-afd7-4c69-b9a0-aaad47d8acc8</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Open'])[2]/following::form[1]</value>
      <webElementGuid>6352ad3d-bcc4-4826-8653-2a16a5cca3c2</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//form[@id = 'registration' and (text() = concat(&quot;DemographicsNameGenderBirthdateContact InfoAddressPhone NumberRelationshipsRelativesConfirm

        

        
        

            
                

                  
                  

                    
                    

                        

                            Name
                            
                                What&quot; , &quot;'&quot; , &quot;s the patient&quot; , &quot;'&quot; , &quot;s name?

                                
                                



    
        Given (required)
    

    No search results.

    
    





    jq(function() {
        jq(&quot;#fr282-field&quot; ).autocomplete({
            source: function( request, response ) {
                jq.ajax({
                    url: &quot;/openmrs/registrationapp/personName/getSimilarNames.action?successUrl=%2Fopenmrs%2Fregistrationapp%2FregisterPatient.page%3FappId%3Dreferenceapplication.registrationapp.registerPatient%26&quot;,
                    dataType: &quot;json&quot;,
                    data: {
                        &quot; , &quot;'&quot; , &quot;searchPhrase&quot; , &quot;'&quot; , &quot;: request.term,
                        &quot; , &quot;'&quot; , &quot;formFieldName&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;givenName&quot; , &quot;'&quot; , &quot;
                    },
                    success: function( data ) {
                        response( jq.map( data.names, function( item ) {
                            return {
                                label: item,
                                value: item
                            }
                        }));
                    }
                });
            },
            minLength: 1
        });
    });




                                
                                



    
        Middle 
    

    No search results.

    
    





    jq(function() {
        jq(&quot;#fr978-field&quot; ).autocomplete({
            source: function( request, response ) {
                jq.ajax({
                    url: &quot;/openmrs/registrationapp/personName/getSimilarNames.action?successUrl=%2Fopenmrs%2Fregistrationapp%2FregisterPatient.page%3FappId%3Dreferenceapplication.registrationapp.registerPatient%26&quot;,
                    dataType: &quot;json&quot;,
                    data: {
                        &quot; , &quot;'&quot; , &quot;searchPhrase&quot; , &quot;'&quot; , &quot;: request.term,
                        &quot; , &quot;'&quot; , &quot;formFieldName&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;middleName&quot; , &quot;'&quot; , &quot;
                    },
                    success: function( data ) {
                        response( jq.map( data.names, function( item ) {
                            return {
                                label: item,
                                value: item
                            }
                        }));
                    }
                });
            },
            minLength: 1
        });
    });




                                
                                



    
        Family Name (required)
    

    No search results.

    
    





    jq(function() {
        jq(&quot;#fr8812-field&quot; ).autocomplete({
            source: function( request, response ) {
                jq.ajax({
                    url: &quot;/openmrs/registrationapp/personName/getSimilarNames.action?successUrl=%2Fopenmrs%2Fregistrationapp%2FregisterPatient.page%3FappId%3Dreferenceapplication.registrationapp.registerPatient%26&quot;,
                    dataType: &quot;json&quot;,
                    data: {
                        &quot; , &quot;'&quot; , &quot;searchPhrase&quot; , &quot;'&quot; , &quot;: request.term,
                        &quot; , &quot;'&quot; , &quot;formFieldName&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;familyName&quot; , &quot;'&quot; , &quot;
                    },
                    success: function( data ) {
                        response( jq.map( data.names, function( item ) {
                            return {
                                label: item,
                                value: item
                            }
                        }));
                    }
                });
            },
            minLength: 1
        });
    });




                                
                            

                            
                            
		                    

                            

                            
                            
                                
                                
                                    
                                    Unidentified Patient
                                
                            

                            


                            
                        

                        

                            Gender

                            



    
        What&quot; , &quot;'&quot; , &quot;s the patient&quot; , &quot;'&quot; , &quot;s gender? (required)
    

    

    
        
        
            Male
        
            Female
        
    

    




    var viewModel = viewModel || {};
    viewModel.validations = viewModel.validations || [];
    viewModel.gender = ko.observable();

    

        
        viewModel.validations.push(function() {
            return jq(&quot; , &quot;'&quot; , &quot;#gender-field&quot; , &quot;'&quot; , &quot;).is(&quot; , &quot;'&quot; , &quot;:disabled&quot; , &quot;'&quot; , &quot;) || (viewModel.gender() ? true : false);
        });
        
    

    



                            
                            
                        

                        

                            Birthdate

                            


    var birthdateDay = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
    var birthdateMonth = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
    var birthdateYear = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;

    jQuery(document).ready(function(){
        

        _.each(jQuery(&quot; , &quot;'&quot; , &quot;.date-component&quot; , &quot;'&quot; , &quot;), function(dateElement){
            jQuery(dateElement).blur(function(){
                var elementValue = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
                if(this.value &amp;&amp; jQuery.trim(this.value).length > 0)
                    elementValue = jQuery.trim(this.value);

                if(this.id == &quot; , &quot;'&quot; , &quot;birthdateDay-field&quot; , &quot;'&quot; , &quot;){
                    if(elementValue.length > 0)
                        birthdateDay = elementValue;
                    else
                        birthdateDay = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
                }else if(this.id == &quot; , &quot;'&quot; , &quot;birthdateMonth-field&quot; , &quot;'&quot; , &quot;){
                    if(elementValue.length > 0)
                        birthdateMonth = elementValue;
                    else
                        birthdateMonth = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
                }else if (this.id == &quot; , &quot;'&quot; , &quot;birthdateYear-field&quot; , &quot;'&quot; , &quot;){
                    if(elementValue.length > 0)
                        birthdateYear = elementValue;
                    else
                        birthdateYear = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
                }

                if (birthdateYear != &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot; &amp;&amp; birthdateMonth != &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot; &amp;&amp; birthdateDay != &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;) {
                    jQuery(&quot; , &quot;'&quot; , &quot;#birthdate-value&quot; , &quot;'&quot; , &quot;).val(birthdateYear+&quot;-&quot;+birthdateMonth+&quot;-&quot;+birthdateDay);
                } else {
                    jQuery(&quot; , &quot;'&quot; , &quot;#birthdate-value&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
                    jQuery(&quot; , &quot;'&quot; , &quot;#birthdateYears-field&quot; , &quot;'&quot; , &quot;).prop(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, false);
                    jQuery(&quot; , &quot;'&quot; , &quot;#birthdateMonths-field&quot; , &quot;'&quot; , &quot;).prop(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, false);
                }
            });
        });
    });




    
        What&quot; , &quot;'&quot; , &quot;s the patient&quot; , &quot;'&quot; , &quot;s birth date? (required)
    
    

    


    
        Day (required)
    
    
    
    
    
    


    



    
        Month (required)
    

    

    
        
            Select
        
        
            January
        
            February
        
            March
        
            April
        
            May
        
            June
        
            July
        
            August
        
            September
        
            October
        
            November
        
            December
        
    

    




    var viewModel = viewModel || {};
    viewModel.validations = viewModel.validations || [];
    viewModel.birthdateMonth = ko.observable();

    

        
        viewModel.validations.push(function() {
            return jq(&quot; , &quot;'&quot; , &quot;#birthdateMonth-field&quot; , &quot;'&quot; , &quot;).is(&quot; , &quot;'&quot; , &quot;:disabled&quot; , &quot;'&quot; , &quot;) || (viewModel.birthdateMonth() ? true : false);
        });
        
    

    



    


    
        Year (required)
    
    
    
    
    
    


            
    
	
    



    jQuery(function() {
        var yearsField = jQuery(&quot; , &quot;'&quot; , &quot;#birthdateYears-field&quot; , &quot;'&quot; , &quot;);
        var monthsField = jQuery(&quot; , &quot;'&quot; , &quot;#birthdateMonths-field&quot; , &quot;'&quot; , &quot;);
        var estimatedField = jQuery(&quot; , &quot;'&quot; , &quot;#birthdateEstimated-field&quot; , &quot;'&quot; , &quot;);

        var skipField = function() {
            e = jQuery.Event(&quot; , &quot;'&quot; , &quot;keydown&quot; , &quot;'&quot; , &quot;);
            e.which = 13; // enter
            yearsField.trigger(e);
        };

        var isExactDate = function() {
            var isEstimatedChecked = jQuery(&quot; , &quot;'&quot; , &quot;#birthdateEstimated-field&quot; , &quot;'&quot; , &quot;).is(&quot; , &quot;'&quot; , &quot;:checked&quot; , &quot;'&quot; , &quot;);
            return (jQuery(&quot; , &quot;'&quot; , &quot;#birthdate-value&quot; , &quot;'&quot; , &quot;).val() != &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot; &amp;&amp; !isEstimatedChecked);
        }

        var toggleEstimatedDate = function() {
            if(isExactDate()) {
                yearsField.val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
                yearsField.prop(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, true);
                monthsField.val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
                monthsField.prop(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, true);
            } else {
                yearsField.prop(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, false);
                monthsField.prop(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, false);
            }
        };

        var clearExactDateFields = function() {
            birthdateDay = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
            birthdateMonth = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
            birthdateYear = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
            jQuery(&quot; , &quot;'&quot; , &quot;#birthdateDay-field&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
            jQuery(&quot; , &quot;'&quot; , &quot;#birthdateMonth-field&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
            jQuery(&quot; , &quot;'&quot; , &quot;#birthdateYear-field&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);

            if (birthdateYear != &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot; &amp;&amp; birthdateMonth != &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot; &amp;&amp; birthdateDay != &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;) {
                jQuery(&quot; , &quot;'&quot; , &quot;#birthdate-value&quot; , &quot;'&quot; , &quot;).val(birthdateYear+&quot;-&quot;+birthdateMonth+&quot;-&quot;+birthdateDay);
            } else {
                jQuery(&quot; , &quot;'&quot; , &quot;#birthdate-value&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
            }
        }

        jQuery(estimatedField).change(function() {
            toggleEstimatedDate();
        });

        jQuery(yearsField).blur(function() {
            var elementValue = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
            if(this.value &amp;&amp; jQuery.trim(this.value).length > 0)
                elementValue = jQuery.trim(this.value);

            if(elementValue.length > 0) {
                birthdateYears = elementValue;
                clearExactDateFields();
            }else {
                birthdateYears = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
            }

        });

        jQuery(monthsField).blur(function() {
            var elementValue = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
            if(this.value &amp;&amp; jQuery.trim(this.value).length > 0)
                elementValue = jQuery.trim(this.value);

            if(elementValue.length > 0) {
                birthdateMonths = elementValue;
                clearExactDateFields();
            }else {
                birthdateMonths = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
            }

        });

        jQuery(yearsField).focus(toggleEstimatedDate);
        jQuery(monthsField).focus(toggleEstimatedDate);
    });

 
Or

    


    
        Estimated Years (required)
    
    
    
    
     year(s)
    



    


    
        Estimated Months (required)
    
    
    
    
     month(s)
    







                        
                    

                    
                    

                    
                  
            
        

            
                

                  
                  

                    
                    

                    
                    

                    
	                        
	                            Address

	                            
	                                
	                            
	                            

	                            
		                            

    
        What is the patient&quot; , &quot;'&quot; , &quot;s address? 
    


        
            
        
            
            	 
					
	        			Address
	        		
	        		
                        
                        
                    
	        	
	        	    
            
        

        
            
        
            
            	 
					
	        			Address 2
	        		
	        		
                        
                        
                    
	        	
	        	    
            
        

        
            
        
            
            	 
					
	        			City/Village
	        		
	        		
                        
                        
                    
	        	
	        	  
            
        
            
        
            
            	 
					
	        			State/Province
	        		
	        		
                        
                        
                    
	        	
	        	  
            
        
            
        
            
            	 
					
	        			Country
	        		
	        		
                        
                        
                    
	        	
	        	  
            
        
            
        
            
            	 
					
	        			Postal Code
	        		
	        		
                        
                        
                    
	        	
	        	    
            
        


		                        
	                        
	                    
                    
	                        
	                            Phone Number

	                            
	                            

	                            
		                            


    
        What&quot; , &quot;'&quot; , &quot;s the patient phone number? 
    
    
    
    
    
    


		                        
	                        
	                    
                    
                  
            
        

            
                

                  
                  

                    
                    

                    
                    

                    
	                        
	                            Relatives

	                            
	                            
	                                    Who is the patient related to?
	                            

	                            
		                            



    
        
            
                Select Relationship Type
                
                    
                        
                        Doctor
                    
                
                    
                        
                        Sibling
                    
                
                    
                        
                        Parent
                    
                
                    
                        
                        Aunt/Uncle
                    
                
                    
                        
                        Supervisor
                    
                
                
                    
                        
                            
                            Patient
                        
                    
                
                    
                
                    
                        
                            
                            Child
                        
                    
                
                    
                        
                            
                            Niece/Nephew
                        
                    
                
                    
                        
                            
                            Supervisee
                        
                    
                
            
        

        
            
    
        Jojo Adhi Sunandar
    

            
        

        
            
                
            
            
                
            
        
    

		                        
	                        
	                    
                    
                  
            
        

        

        
            
            
            
            
            
                There seems to be a patient in the database that exactly matches this one. Please review before confirming:
                
            
            
                Confirm submission?
                
                    
                
                
                    
                
            
        
        &quot;) or . = concat(&quot;DemographicsNameGenderBirthdateContact InfoAddressPhone NumberRelationshipsRelativesConfirm

        

        
        

            
                

                  
                  

                    
                    

                        

                            Name
                            
                                What&quot; , &quot;'&quot; , &quot;s the patient&quot; , &quot;'&quot; , &quot;s name?

                                
                                



    
        Given (required)
    

    No search results.

    
    





    jq(function() {
        jq(&quot;#fr282-field&quot; ).autocomplete({
            source: function( request, response ) {
                jq.ajax({
                    url: &quot;/openmrs/registrationapp/personName/getSimilarNames.action?successUrl=%2Fopenmrs%2Fregistrationapp%2FregisterPatient.page%3FappId%3Dreferenceapplication.registrationapp.registerPatient%26&quot;,
                    dataType: &quot;json&quot;,
                    data: {
                        &quot; , &quot;'&quot; , &quot;searchPhrase&quot; , &quot;'&quot; , &quot;: request.term,
                        &quot; , &quot;'&quot; , &quot;formFieldName&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;givenName&quot; , &quot;'&quot; , &quot;
                    },
                    success: function( data ) {
                        response( jq.map( data.names, function( item ) {
                            return {
                                label: item,
                                value: item
                            }
                        }));
                    }
                });
            },
            minLength: 1
        });
    });




                                
                                



    
        Middle 
    

    No search results.

    
    





    jq(function() {
        jq(&quot;#fr978-field&quot; ).autocomplete({
            source: function( request, response ) {
                jq.ajax({
                    url: &quot;/openmrs/registrationapp/personName/getSimilarNames.action?successUrl=%2Fopenmrs%2Fregistrationapp%2FregisterPatient.page%3FappId%3Dreferenceapplication.registrationapp.registerPatient%26&quot;,
                    dataType: &quot;json&quot;,
                    data: {
                        &quot; , &quot;'&quot; , &quot;searchPhrase&quot; , &quot;'&quot; , &quot;: request.term,
                        &quot; , &quot;'&quot; , &quot;formFieldName&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;middleName&quot; , &quot;'&quot; , &quot;
                    },
                    success: function( data ) {
                        response( jq.map( data.names, function( item ) {
                            return {
                                label: item,
                                value: item
                            }
                        }));
                    }
                });
            },
            minLength: 1
        });
    });




                                
                                



    
        Family Name (required)
    

    No search results.

    
    





    jq(function() {
        jq(&quot;#fr8812-field&quot; ).autocomplete({
            source: function( request, response ) {
                jq.ajax({
                    url: &quot;/openmrs/registrationapp/personName/getSimilarNames.action?successUrl=%2Fopenmrs%2Fregistrationapp%2FregisterPatient.page%3FappId%3Dreferenceapplication.registrationapp.registerPatient%26&quot;,
                    dataType: &quot;json&quot;,
                    data: {
                        &quot; , &quot;'&quot; , &quot;searchPhrase&quot; , &quot;'&quot; , &quot;: request.term,
                        &quot; , &quot;'&quot; , &quot;formFieldName&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;familyName&quot; , &quot;'&quot; , &quot;
                    },
                    success: function( data ) {
                        response( jq.map( data.names, function( item ) {
                            return {
                                label: item,
                                value: item
                            }
                        }));
                    }
                });
            },
            minLength: 1
        });
    });




                                
                            

                            
                            
		                    

                            

                            
                            
                                
                                
                                    
                                    Unidentified Patient
                                
                            

                            


                            
                        

                        

                            Gender

                            



    
        What&quot; , &quot;'&quot; , &quot;s the patient&quot; , &quot;'&quot; , &quot;s gender? (required)
    

    

    
        
        
            Male
        
            Female
        
    

    




    var viewModel = viewModel || {};
    viewModel.validations = viewModel.validations || [];
    viewModel.gender = ko.observable();

    

        
        viewModel.validations.push(function() {
            return jq(&quot; , &quot;'&quot; , &quot;#gender-field&quot; , &quot;'&quot; , &quot;).is(&quot; , &quot;'&quot; , &quot;:disabled&quot; , &quot;'&quot; , &quot;) || (viewModel.gender() ? true : false);
        });
        
    

    



                            
                            
                        

                        

                            Birthdate

                            


    var birthdateDay = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
    var birthdateMonth = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
    var birthdateYear = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;

    jQuery(document).ready(function(){
        

        _.each(jQuery(&quot; , &quot;'&quot; , &quot;.date-component&quot; , &quot;'&quot; , &quot;), function(dateElement){
            jQuery(dateElement).blur(function(){
                var elementValue = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
                if(this.value &amp;&amp; jQuery.trim(this.value).length > 0)
                    elementValue = jQuery.trim(this.value);

                if(this.id == &quot; , &quot;'&quot; , &quot;birthdateDay-field&quot; , &quot;'&quot; , &quot;){
                    if(elementValue.length > 0)
                        birthdateDay = elementValue;
                    else
                        birthdateDay = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
                }else if(this.id == &quot; , &quot;'&quot; , &quot;birthdateMonth-field&quot; , &quot;'&quot; , &quot;){
                    if(elementValue.length > 0)
                        birthdateMonth = elementValue;
                    else
                        birthdateMonth = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
                }else if (this.id == &quot; , &quot;'&quot; , &quot;birthdateYear-field&quot; , &quot;'&quot; , &quot;){
                    if(elementValue.length > 0)
                        birthdateYear = elementValue;
                    else
                        birthdateYear = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
                }

                if (birthdateYear != &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot; &amp;&amp; birthdateMonth != &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot; &amp;&amp; birthdateDay != &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;) {
                    jQuery(&quot; , &quot;'&quot; , &quot;#birthdate-value&quot; , &quot;'&quot; , &quot;).val(birthdateYear+&quot;-&quot;+birthdateMonth+&quot;-&quot;+birthdateDay);
                } else {
                    jQuery(&quot; , &quot;'&quot; , &quot;#birthdate-value&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
                    jQuery(&quot; , &quot;'&quot; , &quot;#birthdateYears-field&quot; , &quot;'&quot; , &quot;).prop(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, false);
                    jQuery(&quot; , &quot;'&quot; , &quot;#birthdateMonths-field&quot; , &quot;'&quot; , &quot;).prop(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, false);
                }
            });
        });
    });




    
        What&quot; , &quot;'&quot; , &quot;s the patient&quot; , &quot;'&quot; , &quot;s birth date? (required)
    
    

    


    
        Day (required)
    
    
    
    
    
    


    



    
        Month (required)
    

    

    
        
            Select
        
        
            January
        
            February
        
            March
        
            April
        
            May
        
            June
        
            July
        
            August
        
            September
        
            October
        
            November
        
            December
        
    

    




    var viewModel = viewModel || {};
    viewModel.validations = viewModel.validations || [];
    viewModel.birthdateMonth = ko.observable();

    

        
        viewModel.validations.push(function() {
            return jq(&quot; , &quot;'&quot; , &quot;#birthdateMonth-field&quot; , &quot;'&quot; , &quot;).is(&quot; , &quot;'&quot; , &quot;:disabled&quot; , &quot;'&quot; , &quot;) || (viewModel.birthdateMonth() ? true : false);
        });
        
    

    



    


    
        Year (required)
    
    
    
    
    
    


            
    
	
    



    jQuery(function() {
        var yearsField = jQuery(&quot; , &quot;'&quot; , &quot;#birthdateYears-field&quot; , &quot;'&quot; , &quot;);
        var monthsField = jQuery(&quot; , &quot;'&quot; , &quot;#birthdateMonths-field&quot; , &quot;'&quot; , &quot;);
        var estimatedField = jQuery(&quot; , &quot;'&quot; , &quot;#birthdateEstimated-field&quot; , &quot;'&quot; , &quot;);

        var skipField = function() {
            e = jQuery.Event(&quot; , &quot;'&quot; , &quot;keydown&quot; , &quot;'&quot; , &quot;);
            e.which = 13; // enter
            yearsField.trigger(e);
        };

        var isExactDate = function() {
            var isEstimatedChecked = jQuery(&quot; , &quot;'&quot; , &quot;#birthdateEstimated-field&quot; , &quot;'&quot; , &quot;).is(&quot; , &quot;'&quot; , &quot;:checked&quot; , &quot;'&quot; , &quot;);
            return (jQuery(&quot; , &quot;'&quot; , &quot;#birthdate-value&quot; , &quot;'&quot; , &quot;).val() != &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot; &amp;&amp; !isEstimatedChecked);
        }

        var toggleEstimatedDate = function() {
            if(isExactDate()) {
                yearsField.val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
                yearsField.prop(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, true);
                monthsField.val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
                monthsField.prop(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, true);
            } else {
                yearsField.prop(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, false);
                monthsField.prop(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, false);
            }
        };

        var clearExactDateFields = function() {
            birthdateDay = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
            birthdateMonth = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
            birthdateYear = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
            jQuery(&quot; , &quot;'&quot; , &quot;#birthdateDay-field&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
            jQuery(&quot; , &quot;'&quot; , &quot;#birthdateMonth-field&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
            jQuery(&quot; , &quot;'&quot; , &quot;#birthdateYear-field&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);

            if (birthdateYear != &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot; &amp;&amp; birthdateMonth != &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot; &amp;&amp; birthdateDay != &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;) {
                jQuery(&quot; , &quot;'&quot; , &quot;#birthdate-value&quot; , &quot;'&quot; , &quot;).val(birthdateYear+&quot;-&quot;+birthdateMonth+&quot;-&quot;+birthdateDay);
            } else {
                jQuery(&quot; , &quot;'&quot; , &quot;#birthdate-value&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
            }
        }

        jQuery(estimatedField).change(function() {
            toggleEstimatedDate();
        });

        jQuery(yearsField).blur(function() {
            var elementValue = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
            if(this.value &amp;&amp; jQuery.trim(this.value).length > 0)
                elementValue = jQuery.trim(this.value);

            if(elementValue.length > 0) {
                birthdateYears = elementValue;
                clearExactDateFields();
            }else {
                birthdateYears = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
            }

        });

        jQuery(monthsField).blur(function() {
            var elementValue = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
            if(this.value &amp;&amp; jQuery.trim(this.value).length > 0)
                elementValue = jQuery.trim(this.value);

            if(elementValue.length > 0) {
                birthdateMonths = elementValue;
                clearExactDateFields();
            }else {
                birthdateMonths = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
            }

        });

        jQuery(yearsField).focus(toggleEstimatedDate);
        jQuery(monthsField).focus(toggleEstimatedDate);
    });

 
Or

    


    
        Estimated Years (required)
    
    
    
    
     year(s)
    



    


    
        Estimated Months (required)
    
    
    
    
     month(s)
    







                        
                    

                    
                    

                    
                  
            
        

            
                

                  
                  

                    
                    

                    
                    

                    
	                        
	                            Address

	                            
	                                
	                            
	                            

	                            
		                            

    
        What is the patient&quot; , &quot;'&quot; , &quot;s address? 
    


        
            
        
            
            	 
					
	        			Address
	        		
	        		
                        
                        
                    
	        	
	        	    
            
        

        
            
        
            
            	 
					
	        			Address 2
	        		
	        		
                        
                        
                    
	        	
	        	    
            
        

        
            
        
            
            	 
					
	        			City/Village
	        		
	        		
                        
                        
                    
	        	
	        	  
            
        
            
        
            
            	 
					
	        			State/Province
	        		
	        		
                        
                        
                    
	        	
	        	  
            
        
            
        
            
            	 
					
	        			Country
	        		
	        		
                        
                        
                    
	        	
	        	  
            
        
            
        
            
            	 
					
	        			Postal Code
	        		
	        		
                        
                        
                    
	        	
	        	    
            
        


		                        
	                        
	                    
                    
	                        
	                            Phone Number

	                            
	                            

	                            
		                            


    
        What&quot; , &quot;'&quot; , &quot;s the patient phone number? 
    
    
    
    
    
    


		                        
	                        
	                    
                    
                  
            
        

            
                

                  
                  

                    
                    

                    
                    

                    
	                        
	                            Relatives

	                            
	                            
	                                    Who is the patient related to?
	                            

	                            
		                            



    
        
            
                Select Relationship Type
                
                    
                        
                        Doctor
                    
                
                    
                        
                        Sibling
                    
                
                    
                        
                        Parent
                    
                
                    
                        
                        Aunt/Uncle
                    
                
                    
                        
                        Supervisor
                    
                
                
                    
                        
                            
                            Patient
                        
                    
                
                    
                
                    
                        
                            
                            Child
                        
                    
                
                    
                        
                            
                            Niece/Nephew
                        
                    
                
                    
                        
                            
                            Supervisee
                        
                    
                
            
        

        
            
    
        Jojo Adhi Sunandar
    

            
        

        
            
                
            
            
                
            
        
    

		                        
	                        
	                    
                    
                  
            
        

        

        
            
            
            
            
            
                There seems to be a patient in the database that exactly matches this one. Please review before confirming:
                
            
            
                Confirm submission?
                
                    
                
                
                    
                
            
        
        &quot;))]</value>
      <webElementGuid>aa050c9b-025b-42f8-8fdb-15b4ad3854e0</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
